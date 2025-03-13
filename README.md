# Anyrun-Websearch-Plus
An [anyrun](https://github.com/Kirottu/anyrun) plugin to search the web through customized search engines. Different from the original [ websearch ](https://github.com/Kirottu/anyrun/tree/master/plugins/websearch) plugin, this modified version allows for setting prefixes for individual search engines.

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
// <Anyrun config dir>/websearch-plus.ron

Config(
  // You can also depend wholely on secondary prefixes by setting the main prefix to "".
  // Note that this will make the default Google engine unusable.
  prefix: "?",

  // The engine to use when to secondary prefix is used
  default_engine: DuckDuckGo,

  // Options:
  // Brave, DuckDuckGo, Ecosia, Github, Qwant, Startpage, Yandex, Custom
  // NOTE: `{}` is replaced by the search query and `https://` is automatically added in front.
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

