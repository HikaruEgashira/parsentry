### Package Distribution

Once packages are submitted to the respective repositories, Parsentry will automatically appear on [repology.org](https://repology.org/project/parsentry/versions).

#### Nix / NixOS

```bash
# Run without installing
nix run github:HikaruEgashira/parsentry

# Install to profile
nix profile install github:HikaruEgashira/parsentry

# Add to flake.nix inputs
{
  inputs.parsentry.url = "github:HikaruEgashira/parsentry";
}
```

For development shell
```bash
nix develop
```

#### Homebrew

##### Setup a Tap (Maintainer)

1. Create a tap repository
   ```bash
   gh repo create homebrew-parsentry --public
   ```

2. Copy the formula
   ```bash
   mkdir -p Formula
   cp packaging/homebrew/parsentry.rb Formula/
   ```

3. Update SHA256 hashes after each release

#### AUR

##### Submit Package (Maintainer)

For binary releases (`parsentry-bin`)
```bash
git clone ssh://aur@aur.archlinux.org/parsentry-bin.git
cp packaging/aur/PKGBUILD parsentry-bin/
cd parsentry-bin
makepkg --printsrcinfo > .SRCINFO
git add PKGBUILD .SRCINFO
git commit -m "Update to version X.Y.Z"
git push
```

For git builds (`parsentry-git`)
```bash
git clone ssh://aur@aur.archlinux.org/parsentry-git.git
cp packaging/aur/PKGBUILD-git parsentry-git/PKGBUILD
cd parsentry-git
makepkg --printsrcinfo > .SRCINFO
git add PKGBUILD .SRCINFO
git commit -m "Initial commit"
git push
```
