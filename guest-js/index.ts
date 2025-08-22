import { invoke ,addPluginListener, PluginListener } from "@tauri-apps/api/core"

export type PushTokenResponse = {
  value?: string;
}
export type PushPermissionResponse = {
  granted: boolean;
}

export async function getFcmToken(): Promise<string | null> {
  return await invoke<{value?: string}>('plugin:push-notifications|get_fcm_token', {
    payload: {},
  }).then((r: PushTokenResponse) => (r.value ? r.value : null));
}

export async function getApnsToken(): Promise<string | null> {
  return await invoke<{value?: string}>('plugin:push-notifications|get_apns_token', {
    payload: {},
  }).then((r: PushTokenResponse) => (r.value ? r.value : null));
}

export async function requestPushPermission(): Promise<boolean> {
  return await invoke<{granted: boolean}>('plugin:push-notifications|requestPermissions', {
    payload: {},
  }).then((r: PushPermissionResponse) => (r.granted));
}

export async function checkPushPermission(): Promise<boolean> {
  return await invoke<{granted: boolean}>('plugin:push-notifications|checkPermissions', {
    payload: {},
  }).then((r: PushPermissionResponse) => (r.granted));
}

export async function onNewFcmToken(
  handler: (token: string) => void
): Promise<PluginListener> {
  return await addPluginListener(
    'push-notifications',
    'new_fcm_token',
    handler
  );
}

export async function onNewApnsToken(
  handler: (token: string) => void
): Promise<PluginListener> {
  return await addPluginListener(
    'push-notifications',
    'new_apns_token',
    handler
  );
}
