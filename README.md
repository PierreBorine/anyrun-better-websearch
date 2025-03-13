# Anyrun-Better-Websearch
An [Anyrun](https://github.com/Kirottu/anyrun) plugin to search the web through customized search engines.

## Features
This plugin is a fork of [anyrun-websearch-plus](https://github.com/kuokuo123/anyrun-websearch-plus) with the following additional features compared to the original [websearch](https://github.com/Kirottu/anyrun/tree/master/plugins/websearch).

- A Nix flake.
  <details>
  <summary>Usage</summary>

  ### Add to the flake
  ```Nix
    # flake.nix
    # ...
    inputs = {
      anyrun-better-websearch.url = "github:PierreBorine/anyrun-better-websearch";
    };
    # ...
  ```

  ### Use the package
  ```Nix
    # home.nix
    # ...
    programs.anyrun.config.plugins = [
      inputs.anyrun-better-websearch.packages.${pkgs.system}.default
    ];
    # ...
  ```

  </details>
- Secondary prefixes to easily search with a specific engine.
- Fuzzy matched prefixes for more flexible search.

## Usage

Enter your search term with a prefix to call for the search engine. If more than two search engines have the same prefix, select the search action you want with arrow keys.

*Examples following the config below.*

With only the main prefix, the default engine is used<br>
<kbd>? hyprland</kbd> → "hyprland" with the default

Can use the secondary prefix to quickly use a specific engine<br>
<kbd>?noogle hyprland</kbd> → "hyprland" on Noogle

Secondary prefixes are fuzzy matched<br>
<kbd>?ngl hyprland</kbd> → "hyprland" on Noogle

If multiple engines are matched, you can select the one you want<br>
<kbd>?nix hyprland</kbd> → "hyprland" on Nixpkgs or NixOS Options

## Configuration

An example config.

```ron
// <Anyrun config dir>/better-websearch.ron

Config(
  // You can also depend wholely on secondary prefixes by setting the main prefix to "".
  // Note that this will make the default Google engine unusable.
  prefix: "?",

  // Options:
  // Brave, DuckDuckGo, Ecosia, Github, Qwant, Startpage, Yandex, Custom
  // NOTE: `{}` is replaced by the search query and `https://` is automatically added in front.

  // The engine to use when no secondary prefix is used
  default_engine: DuckDuckGo,

  // Additional engines to use
  engines: [
      Github,
      Brave,
      Custom(
        name: "Nixpkgs",
        url: "search.nixos.org/packages?query={}",
        secondary_prefix: "nixpkgs",
      ),
      Custom(
        name: "Home Manager",
        url: "home-manager-options.extranix.com/?query={}",
        secondary_prefix: "home",
      ),
      Custom(
        name: "NixOS Options",
        url: "search.nixos.org/options?query={}",
        secondary_prefix: "nixopts",
      ),
      Custom(
        name: "Noogle",
        url: "noogle.dev/q?term={}",
        secondary_prefix: "noogle",
      ),
  ],
)
```

