```
remote v1.0.0
Casey Rodarmor <casey@rodarmor.com>
Generate remote repo URLs - https://github.com/casey/remote

USAGE:
    remote <service> <user> <project>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <service>     [possible values: bitbucket-hg, bitbucket-https, bitbucket-ssh, github-https, github-ssh, gitlab
                 -https, gitlab-ssh, pikacode-https, pikacode-ssh]
    <user>       
    <project>    

DESCRIPTION:

    A little program that prints repository URLs.

    On its own it does not save a great deal of typing, but you can create
    aliases in your shell's configuration file like so:

    alias github=`remote github-ssh gazebo`
    alias bitbucket=`remote bitbucket-ssh gazebo`

    Assuming you have the username `gazebo` on both github and bitbucket, you
    can then clone your own repositories easily:

    $ git clone `github foo`
    Cloning into 'foo'...
    ...

    Or add new remotes to existing repos:

    $ git remote add `github foo`

    And of course you can always use remote directly:

    $ git clone `remote github-ssh rust-lang cargo`
    Cloning into 'cargo'...
    ...
```
