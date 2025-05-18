# Configuration

The configuration can bei either done by API and/or configuration file.
Supported file types for the configuration file are `JSON`, `YAML` and `XML`.
The configuration file must be named `fastlogging.<EXT>`.
The location must be either the current working directory or defined by the
environment variable `FASTLOGGING_CONFIG_FILE`.

An example for the configuration file in `JSON`:

```json
{
  "level": 0,
  "domain": "root",
  "hostname": "bender",
  "pname": "write_config_file",
  "pid": 935659,
  "tname": false,
  "tid": false,
  "structured": "String",
  "console": {
    "level": 40,
    "colors": true
  },
  "file": {
    "level": 10,
    "path": "/tmp/write_config_file.log",
    "size": 1048576,
    "backlog": 4,
    "timeout": {
      "secs": 3600,
      "nanos": 0
    },
    "time": {
      "secs_since_epoch": 1717081855,
      "nanos_since_epoch": 211877680
    },
    "compression": "Deflate"
  },
  "server": null,
  "connect": {
    "level": 50,
    "address": "127.0.0.1:12346",
    "port": 12346,
    "key": "NONE"
  },
  "syslog": null,
  "level2sym": "Sym"
}
```

The same in `YAML`:

```yaml
level: 0
domain: root
hostname: bender
pname: write_config_file
pid: 935659
tname: false
tid: false
structured: String
console:
  level: 40
  colors: true
file:
  level: 10
  path: /tmp/write_config_file.log
  size: 1048576
  backlog: 4
  timeout:
    secs: 3600
    nanos: 0
  time:
    secs_since_epoch: 1717081855
    nanos_since_epoch: 211877680
  compression: Deflate
server: null
connect:
  level: 50
  address: 127.0.0.1:12346
  port: 12346
  key: NONE
syslog: null
level2sym: Sym
```

The same in `XML`:

```xml
<FileConfig>
    <level>0</level>
    <domain>root</domain>
    <hostname>bender</hostname>
    <pname>write_config_file</pname>
    <pid>935659</pid>
    <tname>false</tname>
    <tid>false</tid>
    <structured>String</structured>
    <console>
        <level>40</level>
        <colors>true</colors>
    </console>
    <file>
        <level>10</level>
        <path>/tmp/write_config_file.log</path>
        <size>1048576</size>
        <backlog>4</backlog>
        <timeout>
            <secs>3600</secs>
            <nanos>0</nanos>
        </timeout>
        <time>
            <secs_since_epoch>1717081855</secs_since_epoch>
            <nanos_since_epoch>211877680</nanos_since_epoch>
        </time>
        <compression>Deflate</compression>
    </file>
    <server />
    <connect>
        <level>50</level>
        <address>127.0.0.1:12346</address>
        <port>12346</port>
        <key>NONE</key>
    </connect>
    <syslog />
    <level2sym>Sym</level2sym>
</FileConfig>
```
