macro_rules! test {
  ($name:ident, $service:expr, $user:expr, $project:expr, $expected:expr) => {
    #[test]
    fn $name() {
      let result = super::run(&["remote", $service, $user, $project]);
      if result != $expected {
        panic!("{} != {}", result, $expected);
      }
    }
  };
}

test! {
  github_ssh, "github-ssh", "casey", "remote", "git@github.com:casey/remote.git"
}

test! {
  bitbucket_https, "bitbucket-https", "casey", "remote", "https://bitbucket.org/casey/remote.git"
}
