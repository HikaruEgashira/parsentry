### Package Distribution

Once packages are submitted to the respective repositories, Parsentry will automatically appear on [repology.org](https://repology.org/project/parsentry/versions).

#### crates.io

##### Install (User)

```bash
# Install from crates.io
cargo install parsentry

# Install from GitHub (latest)
cargo install --git https://github.com/HikaruEgashira/parsentry
```

##### Publish Package (Maintainer)

1. Ensure you're logged in to crates.io
   ```bash
   cargo login
   ```

2. Verify package before publishing
   ```bash
   cargo publish --dry-run
   ```

3. Publish to crates.io
   ```bash
   cargo publish
   ```

4. Verify the package appears on [crates.io/crates/parsentry](https://crates.io/crates/parsentry)

**Note**: Version bumps in `Cargo.toml` must be done before publishing. Once published, a version cannot be changed or deleted.

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
