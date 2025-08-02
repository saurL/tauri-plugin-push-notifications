import { invoke } from "@tauri-apps/api/core"

export type PushTokenResponse = {
  value?: string;
}
export type PushPermissionResponse = {
  granted: boolean;
}

export async function pushToken(): Promise<string | null> {
  return await invoke<{value?: string}>('plugin:push-notifications|push_token', {
    payload: {},
  }).then((r: PushTokenResponse) => (r.value ? r.value : null));
}

export async function request_push_permission(): Promise<boolean> {
  return await invoke<{granted: boolean}>('plugin:push-notifications|request_push_permission', {
    payload: {},
  }).then((r: PushPermissionResponse) => (r.granted));
}
 