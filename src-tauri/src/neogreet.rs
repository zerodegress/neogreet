use std::{collections::HashMap, path::PathBuf};

use anyhow::anyhow;
use greetd_ipc::{codec::TokioCodec, AuthMessageType, ErrorType, Request, Response};
use serde::{Deserialize, Serialize};
use tauri::State;
use tokio::net::UnixStream;

pub mod cli;

fn greetd_sock_path() -> PathBuf {
  std::env::var("GREETD_SOCK")
    .expect("cannot get $GREETD_SOCK, it is required.")
    .into()
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionType {
  X11,
  Wayland,
}

async fn login(startup: Startup, user: &str, password: Option<&str>) -> anyhow::Result<bool> {
  let mut stream = UnixStream::connect(greetd_sock_path()).await?;

  let mut next_request = Request::CreateSession {
    username: user.to_string(),
  };
  let mut starting = false;
  loop {
    next_request.write_to(&mut stream).await?;

    match Response::read_from(&mut stream).await? {
      Response::AuthMessage {
        auth_message,
        auth_message_type,
      } => {
        let response = match auth_message_type {
          AuthMessageType::Visible => todo!("visible question not impl."),
          AuthMessageType::Secret => password.map(|p| p.to_string()),
          AuthMessageType::Info => {
            eprintln!("info: {auth_message}");
            None
          }
          AuthMessageType::Error => {
            eprintln!("error: {auth_message}");
            None
          }
        };

        next_request = Request::PostAuthMessageResponse { response };
      }
      Response::Success => {
        if starting {
          return Ok(true);
        } else {
          starting = true;
          next_request = Request::StartSession {
            env: vec![],
            cmd: match startup {
              Startup::DesktopEntry {
                path: ref desktop,
                ref session_type,
              } => {
                let cmd = freedesktop_desktop_entry::DesktopEntry::from_path(
                  desktop,
                  Some(freedesktop_desktop_entry::get_languages_from_env().as_slice()),
                )?
                .exec()
                .ok_or_else(|| anyhow!("incorrect desktop"))?
                .to_string();
                match session_type {
                  SessionType::Wayland => vec![cmd],
                  SessionType::X11 => vec!["startx".to_string(), cmd],
                }
              }
              _ => todo!(),
            },
          }
        }
      }
      Response::Error {
        error_type,
        description,
      } => {
        Request::CancelSession.write_to(&mut stream).await?;
        match error_type {
          ErrorType::AuthError => return Ok(false),
          ErrorType::Error => return Err(anyhow!("login error: {description:?}")),
        }
      }
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Startup {
  DesktopEntry {
    path: PathBuf,
    session_type: SessionType,
  },
  Command(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Defaults {
  pub startup: Option<Startup>,
  pub user: Option<String>,
}

#[tauri::command]
pub fn neogreet_available_wayland_desktops() -> Vec<PathBuf> {
  let locales = freedesktop_desktop_entry::get_languages_from_env();

  freedesktop_desktop_entry::Iter::new([PathBuf::from("/usr/share/wayland-sessions")].into_iter())
    .entries(Some(&locales))
    .map(|entry| entry.path)
    .collect()
}

#[tauri::command]
pub fn neogreet_available_x11_desktops() -> Vec<PathBuf> {
  let locales = freedesktop_desktop_entry::get_languages_from_env();

  freedesktop_desktop_entry::Iter::new([PathBuf::from("/usr/share/xsessions")].into_iter())
    .entries(Some(&locales))
    .map(|entry| entry.path)
    .collect()
}

#[tauri::command]
pub fn neogreet_available_desktops() -> Vec<PathBuf> {
  let locales = freedesktop_desktop_entry::get_languages_from_env();

  freedesktop_desktop_entry::Iter::new(
    [
      PathBuf::from("/usr/share/xsessions"),
      PathBuf::from("/usr/share/wayland-sessions"),
    ]
    .into_iter(),
  )
  .entries(Some(&locales))
  .map(|entry| entry.path)
  .collect()
}

#[tauri::command]
pub fn neogreet_desktops_name_map(languages: Option<Vec<String>>) -> HashMap<PathBuf, String> {
  let locales = languages.unwrap_or(freedesktop_desktop_entry::get_languages_from_env());

  freedesktop_desktop_entry::Iter::new(
    [
      PathBuf::from("/usr/share/xsessions"),
      PathBuf::from("/usr/share/wayland-sessions"),
    ]
    .into_iter(),
  )
  .entries(Some(&locales))
  .map(|entry| {
    (
      entry.path.to_owned(),
      entry
        .name(&locales)
        .unwrap_or(entry.path.to_string_lossy())
        .to_string(),
    )
  })
  .collect()
}

#[tauri::command]
pub fn neogreet_defaults(cli: State<'_, cli::Cli>) -> Defaults {
  Defaults {
    startup: None,
    user: cli.user.to_owned(),
  }
}

#[tauri::command]
pub async fn neogreet_login(
  startup: Startup,
  user: &str,
  password: Option<&str>,
) -> Result<bool, String> {
  login(startup, user, password)
    .await
    .map_err(|err| err.to_string())
}
