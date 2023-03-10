import { redirect } from '@sveltejs/kit';
import { fetchGithub } from '~/lib/helpers';
import type { TSession } from '~/lib/types';
import type { LayoutServerLoad } from '../$types';

export const load: LayoutServerLoad = async (event) => {
	const session = await event.locals.getSession();
	if (!session?.user) {
		throw redirect(303, '/');
	}

	try {
		const { accessToken } = session as TSession;
		const { login } = await fetchGithub(`https://api.github.com/user`, accessToken);
		const githubEvents = await fetchGithub(
			`https://api.github.com/users/${login}/events?per_page=100`,
			accessToken
		);
		return { githubEvents };
	} catch {
		throw redirect(303, '/');
	}
};
