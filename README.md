# Anyrun-Websearch-Plus
An [anyrun](https://github.com/Kirottu/anyrun) plugin to search the web through customized search engines. Different from the original [ websearch ](https://github.com/Kirottu/anyrun/tree/master/plugins/websearch) plugin, this modified version allows for setting prefixes for individual search engines.

## Usage

Enter your search term with a prefix to call for the search engine. If more than two search engines have the same prefix, select the search action you want with arrow keys.

With only the main prefix, Google is used by default<br>
<kbd>? hyprland</kbd> → "hyprland" on Google

Can use the secondary prefix to quickly use a specific engine<br>
<kbd>?awiki hyprland</kbd> → "hyprland" on the Arch wiki

Secondary prefixes are fuzzy matched<br>
<kbd>?aw hyprland</kbd> → "hyprland" on the Arch wiki

If multiple engines are matched, you can select the one you want<br>
<kbd>?a hyprland</kbd> → "hyprland" on the Arch Package Repo / Arch Wiki / AUR

## Configuration

An example config.

```ron
// <Anyrun config dir>/websearch_plus.ron

Config(
  // You can also depend wholely on secondary prefixes by setting the main prefix to "".
  // Note that this will make the default Google engine unusable.

  prefix: "?",

  // Options: Google, Custom
  // NOTE: `{}` is replaced by the search query and `https://` is automatically added in front.

  engines: [
    // Example config:

    Custom(
      name: "DuckDuckGo",
      url: "duckduckgo.com/?q={}",
      secondary_prefix: "d",
    ),

    Custom(
      name: "Arch Package Repository",
      url: "archlinux.org/packages/?q={}",
      secondary_prefix: "apkg",
    ),

    Custom(
      name: "Archwiki",
      url: "wiki.archlinux.org/index.php?search={}",
      secondary_prefix: "awiki",
    ),

    Custom(
      name: "Arch User Repository",
      url: "aur.archlinux.org/packages?K={}",
      secondary_prefix: "aur",
    ),

    Custom(
      name: "Github",
      url: "github.com/search?q={}",
      secondary_prefix: "gh",
    ),

    Custom(
      name: "Reddit",
      url: "www.reddit.com/search/?q={}",
      secondary_prefix: "rd",
    ),

    Custom(
      name: "Youtube",
      url: "www.youtube.com/results?search_query={}",
      secondary_prefix: "yt",
    ),

    Google,
  ],
)
```

