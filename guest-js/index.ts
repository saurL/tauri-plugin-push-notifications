import { invoke } from "@tauri-apps/api/core"

export type PushTokenResponse = {
  value?: String;
}

export async function pushToken(): Promise<string | null> {
  return await invoke<{value?: string}>('plugin:push-notifications|get-push-token', {
    payload: {},
  }).then((r: PushTokenResponse) => (r.value ? r.value : null));
}
