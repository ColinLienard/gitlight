import type { Session } from '@auth/core/types';
import type { ComponentType } from 'svelte';
import type { TGithubLabel } from './github-types';

export type TSession = Session & { accessToken: string };

export type TColors = 'blue' | 'purple' | 'green' | 'red' | 'grey';

export type TEvent = {
	id: string;
	read: boolean;
	pinned: boolean;
	title: string;
	description: (
		| {
				text: string;
				image?: string;
				icon?: ComponentType;
				iconColor?: TColors;
		  }
		| string
	)[];
	time: string;
	icon: ComponentType;
	iconColor?: TColors;
	repo: string;
	number?: number;
	labels?: TGithubLabel[];
	url?: string;
};
