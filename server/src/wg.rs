use std::io::Write;
use std::process::Stdio;

fn wg_install() -> anyhow::Result<()> {
    crate::sudo()?;
    let out = std::process::Command::new("wg").output()?;
    if !out.status.success() {
        print_stderr(&out);
        return Err(anyhow::anyhow!(
            "you need to install the wireguard toolset."
        ));
    }
    Ok(())
}

fn print_stderr(out: &std::process::Output) {
    if !out.status.success() {
        String::from_utf8_lossy(out.stderr.as_slice())
            .lines()
            .for_each(|s| println!("stderr: {}", s));
    }
}

pub struct WireGuardCommand;

impl WireGuardCommand {
    pub fn generate_key_pair(psk: bool) -> anyhow::Result<WgKeyPair> {
        wg_install()?;
        let prv_out = std::process::Command::new("wg").arg("genkey").output()?;
        if !prv_out.status.success() {
            print_stderr(&prv_out);
        }

        let prv_key = WireGuardCommand::output_to_key(prv_out)?;

        let mut pub_out = std::process::Command::new("wg")
            .arg("pubkey")
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()?;

        pub_out
            .stdin
            .as_mut()
            .expect("Failed to get stdin for wg pubkey")
            .write_all(prv_key.as_bytes())?;

        let pub_out = pub_out.wait_with_output()?;
        if !pub_out.status.success() {
            print_stderr(&pub_out)
        }

        let pub_key = WireGuardCommand::output_to_key(pub_out)?;
        let mut pair = WgKeyPair::new(pub_key, prv_key);
        if psk {
            let psk_out = std::process::Command::new("wg").arg("genpsk").output()?;
            if !psk_out.status.success() {
                print_stderr(&psk_out)
            }
            let ps_key = WireGuardCommand::output_to_key(psk_out)?;
            pair.set_ps_key(Some(ps_key))
        }
        Ok(pair)
    }

    fn output_to_key(output: std::process::Output) -> anyhow::Result<String> {
        let string = String::from_utf8(output.stdout)?.trim().to_string();
        Ok(string)
    }
}

#[derive(Debug)]
pub struct WgKeyPair {
    public_key: String,
    private_key: String,
    ps_key: Option<String>,
}

impl WgKeyPair {
    pub fn new(public_key: String, private_key: String) -> Self {
        Self {
            public_key,
            private_key,
            ps_key: None,
        }
    }

    pub fn public_key(&self) -> &str {
        &self.public_key
    }
    pub fn private_key(&self) -> &str {
        &self.private_key
    }
    pub fn set_ps_key(&mut self, ps_key: Option<String>) {
        self.ps_key = ps_key;
    }
}
