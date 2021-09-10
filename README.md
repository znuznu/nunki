# nunki
Collects TODOs in source code and reports them as issues.

## Usage

```
Nunki CLI 0.1.0
Arthur 'znu' F.
Collects TODOs in source code and reports them as issues.

USAGE:
    nunki [OPTIONS] -m <mode>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m <mode>
            The execution mode.
            
            Set to `match`: extract all untracked todos of the project and print them, without affecting anything.
            
            Set to `patch`: extract all untracked todos and ask to create an issue related to these todos on the remote
            (Github) repository. The issue # is then assigned directly in the source code, ready to be committed.
            [default: match]  [possible values: Match, Patch]
    -p <path>        The source code entrypoint (directory or file).
```

## TODOs regex
There's currently no way to add customs regex.

### Untracked TODO

	^(.*)TODO[: ]? (.*)(\n)?$

## Configuration file
A `nunki.toml` file is required at the root of the project.  

### Sections

__Remote__  
`<name>*`: the repository remote name 

__Example__

```
[remote]
name = "origin"
```

__Ignore__  
`<paths>`: an array containing the paths to exclude

__Example__

```
[ignore]
paths = [ "src/folderA/file.rs", "src/folderB" ]
```

_* required_

# Token

## Github

A Github personal token is needed. Just add it to your `.bashrc`, `.zshrc` or whatever.

__Example__

    export GITHUB_PERSONAL_ACCESS_TOKEN=<token>

See the documentation [here](https://docs.github.com/en/github/authenticating-to-github/keeping-your-account-and-data-secure/creating-a-personal-access-token).

## References
* https://docs.github.com/en/rest/reference/issues
* The project is inspired by https://github.com/tsoding/snitch

_Sigma Sagittarii, Latinized from σ Sagittarii; formally named [Nunki](https://en.wikipedia.org/wiki/Sigma_Sagittarii) /ˈnʌŋki/, is the second-brightest star in the constellation of Sagittarius._

