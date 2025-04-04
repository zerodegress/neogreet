import { invoke } from '@tauri-apps/api/core'

export type Startup = {
  type: 'desktop_entry'
  path: string
  session_type: 'wayland' | 'x11'
}

export const login = async (
  startup: Startup,
  user: string,
  password: string | undefined = undefined
): Promise<boolean> => {
  return await invoke('neogreet_login', {
    startup,
    user,
    password,
  })
}

export const defaults = async (): Promise<{
  user?: string
  startup?: Startup
}> => {
  return await invoke('neogreet_defaults')
}

export const availableX11Desktops = async (): Promise<string[]> => {
  return await invoke('neogreet_available_x11_desktops')
}

export const availableWaylandDesktops = async (): Promise<string[]> => {
  return await invoke('neogreet_available_wayland_desktops')
}

export const desktopsNameMap = async (): Promise<Record<string, string>> => {
  return await invoke('neogreet_desktops_name_map')
}
