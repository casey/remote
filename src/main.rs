extern crate clap;

use clap::{App, Arg, AppSettings};

type STR = &'static str;

#[derive(Clone, Copy)]
enum Service {
  BitbucketHG,
  BitbucketHTTPS,
  BitbucketSSH,
  GitHubHTTPS,
  GitHubSSH,
  GitLabHTTPS,
  GitLabSSH,
}

use Service::*;

impl Service {
  fn name(self) -> STR {
    match self {
      BitbucketHG    => "bitbucket-hg",
      BitbucketHTTPS => "bitbucket-https",
      BitbucketSSH   => "bitbucket-ssh",
      GitHubHTTPS    => "github-https",
      GitHubSSH      => "github-ssh",
      GitLabHTTPS    => "gitlab-https",
      GitLabSSH      => "gitlab-ssh",
    }
  }

  fn from_name(name: &str) -> Option<Service> {
    match name {
      "bitbucket-hg"    => Some(BitbucketHG),
      "bitbucket-https" => Some(BitbucketHTTPS),
      "bitbucket-ssh"   => Some(BitbucketSSH),
      "github-https"    => Some(GitHubHTTPS),
      "github-ssh"      => Some(GitHubSSH),
      "gitlab-https"    => Some(GitLabHTTPS),
      "gitlab-ssh"      => Some(GitLabSSH),
      _                 => None,
    }
  }

  fn template(self) -> Template {
    match self {
      BitbucketHG    => Template::new("ssh://hg@bitbucket.org/", "/", ""),
      BitbucketHTTPS => Template::new("https://bitbucket.org/",  "/", ".git"),
      BitbucketSSH   => Template::new("git@bitbucket.org:",      "/", ".git"),
      GitHubHTTPS    => Template::new("https://github.com/",     "/", ".git"),
      GitHubSSH      => Template::new("git@github.com:",         "/", ".git"),
      GitLabHTTPS    => Template::new("https://gitlab.com/",     "/", ".git"),
      GitLabSSH      => Template::new("git@gitlab.com:",         "/", ".git"),
    }
  }
}

#[derive(Copy, Clone)]
struct Template {
  prefix:  STR,
  infix:   STR,
  postfix: STR,
}

impl Template {
  fn new(prefix: STR, infix: STR, postfix: STR) -> Template {
    Template {
      prefix:  prefix,
      infix:   infix,
      postfix: postfix,
    }
  }

  fn instantiate(self, user: &str, project: &str) -> String {
    [self.prefix, user, self.infix, project, self.postfix].join("")
  }
} 

fn run<I, T>(args: I) -> String
  where I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
{
  let services = &[
    BitbucketHG,
    BitbucketHTTPS,
    BitbucketSSH,
    GitHubHTTPS,
    GitHubSSH,
    GitLabHTTPS,
    GitLabSSH,
  ];

  let service_names = services.iter().map(|service| service.name()).collect::<Vec<_>>();

  let matches = App::new("remote")
    .version(concat!("v", env!("CARGO_PKG_VERSION")))
    .author("Casey Rodarmor <casey@rodarmor.com>")
    .about("Generate remote repo URLs - https://github.com/casey/remote")
    .setting(AppSettings::ColoredHelp)
    .arg(Arg::with_name("service")
         .possible_values(&service_names)
         .required(true))
    .arg(Arg::with_name("user")
         .required(true))
    .arg(Arg::with_name("project")
         .required(true))
    .get_matches_from(args);

  let service_name = matches.value_of("service").expect("service argument had no value");
  let service      = Service::from_name(service_name).expect("service name had bad value");
  let user         = matches.value_of("user").expect("user argument had no value");
  let project      = matches.value_of("project").expect("project argument had no value");

  service.template().instantiate(user, project)
}

fn main() {
  println!("{}", run(std::env::args_os()));
}

#[cfg(test)]
mod tests;
