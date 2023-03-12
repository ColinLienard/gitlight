import { writable } from 'svelte/store';
import type { TEvent } from '~/lib/types';

export const githubEvents = writable<TEvent[]>([]);
