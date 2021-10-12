Reads an RSS feed from the `action=rss` pmWiki endpoint and formats it to post
to a discord webhook endpoint.

This bot has two persistence files:
* `config.json`: gives hint as to where to read the RSS feed, which password to
  use and where to post the processed data. See [discord
  documentation](https://support.discord.com/hc/en-us/articles/228383668)
* `sent.json`: records the entries (tbh, only their `pubDate`) that were
  processed and sent to the channel so that they can be filtered out of the
  messages to send to discord

`config.json`:
```json
{
    "rss_url": "https://example.com/wiki/pmwiki.php/Site/AllRecentChanges?action=rss",
    "hook_url": "https://discord.com/api/webhooks/XYZ",
    "pass": "password"
}
```

## Usage

`./pmwiki-rss-discord-bot` (make sure you have the `config.json` and
`sent.json` in the working directory)

## License

This program is literally 80 lines of glue code. You do wtf you want with it.

Copyright © 2000 Gibonus
This work is free. It comes without any warranty, to the extent permitted by
applicable law. You can redistribute it and/or modify it under the
terms of the Do What The Fuck You Want To Public License, Version 2,
as published by Sam Hocevar. See next lines.

DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
Version 2, December 2004

Copyright 2021 gibonus

Everyone is permitted to copy and distribute verbatim or modified
copies of this license document, and changing it is allowed as long
as the name is changed.

DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

0. You just DO WHAT THE FUCK YOU WANT TO.
