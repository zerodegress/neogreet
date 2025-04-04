import { createResource, createSignal, For, onCleanup, onMount } from 'solid-js'
import './App.css'
import dayjs from 'dayjs'
import {
  availableX11Desktops,
  availableWaylandDesktops,
  defaults,
  desktopsNameMap,
  login,
  Startup,
} from './neogreet'
export const App = () => {
  const [user, setUser] = createSignal('')
  const [startup, setStartup] = createSignal<Startup | undefined>(undefined)
  const [password, setPassword] = createSignal('')
  const [date, setDate] = createSignal(dayjs())
  const [x11Desktops] = createResource(availableX11Desktops, {
    initialValue: [],
  })
  const [waylandDesktops] = createResource(availableWaylandDesktops, {
    initialValue: [],
  })
  const desktops = () => [...x11Desktops(), ...waylandDesktops()]
  const desktopsSessionTypeMap = () =>
    Object.fromEntries([
      ...x11Desktops().map(x => [x, 'x11']),
      ...waylandDesktops().map(x => [x, 'wayland']),
    ]) as Record<string, 'x11' | 'wayland'>
  const [desktopsNameMap_] = createResource(() => desktopsNameMap(), {
    initialValue: {},
  })
  const [defaults_] = createResource(
    async () => {
      const defaults_ = await defaults()
      setUser(prev => defaults_.user || prev)
      setStartup(defaults_.startup)
      return defaults_
    },
    {
      initialValue: {
        startup: undefined,
        user: undefined,
      },
    }
  )

  onMount(() => {
    const update = setInterval(() => {
      setDate(dayjs())
    }, 1000)
    onCleanup(() => clearInterval(update))
  })

  return (
    <div id='neogreeter'>
      <div class='status-bar'>
        <span>{date().format('HH:mm:ss')}</span>
        <div class='power-button'>⏻</div>
      </div>

      <div class='login-container'>
        <div class='user-avatar'>👤</div>

        <div class='input-container'>
          <input
            type='text'
            class='text-input'
            placeholder='Username'
            value={user()}
            onInput={ev => setUser(ev.target.value)}
          />
        </div>

        <div class='input-container'>
          <select
            class='desktop-select'
            onInput={ev =>
              setStartup({
                type: 'desktop_entry',
                path: ev.target.value,
                session_type: desktopsSessionTypeMap()[ev.target.value],
              })
            }
          >
            <option value=''>Please Select</option>
            <For each={desktops()}>
              {desktop => (
                <option
                  selected={
                    defaults_().startup?.type == 'desktop_entry' &&
                    defaults_().startup?.path == desktop
                  }
                  value={desktop}
                >
                  {desktopsNameMap_()[desktop]}
                </option>
              )}
            </For>
          </select>
        </div>

        <div class='input-container'>
          <input
            type='password'
            class='password-input'
            placeholder='Password'
            value={password()}
            onInput={e => setPassword(e.target.value)}
          />
        </div>

        <button
          class='login-button'
          onClick={async () => {
            try {
              const startup_ = startup()
              const user_ = user()
              const password_ = password()
              if (!startup_) {
                throw new Error('No startup specified')
              }
              if (!user_) {
                throw new Error('No user specified')
              }
              if (!password_) {
                throw new Error('No password')
              }
              if (!(await login(startup_, user_, password_))) {
                alert('failed')
              }
            } catch (err) {
              alert(err)
            }
          }}
        >
          Sign In
        </button>
      </div>
    </div>
  )
}
