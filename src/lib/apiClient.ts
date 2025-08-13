import { invoke } from "@tauri-apps/api/core";

const VITE_API_URL = import.meta.env.VITE_API_URL as string;

const isTauri = (window as any).__TAURI__ !== undefined;

async function callApi<T>(command: string, args?: any): Promise<T> {
    if (isTauri) {
        return await invoke<T>(command, args);
    }

    let url = `${VITE_API_URL}/${command.replace(/_/g, '/')}`;
    const method = 'GET'; // TODO: determine method based on command

    const options: RequestInit = {
        method,
        headers: {
            'Content-Type': 'application/json',
        },
    };

    if (args) {
        if (method === 'GET') {
            const params = new URLSearchParams(args);
            url += `?${params}`;
        } else {
            options.body = JSON.stringify(args);
        }
    }

    const response = await fetch(url, options);
    if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
    }
    return await response.json();
}

export { callApi };
