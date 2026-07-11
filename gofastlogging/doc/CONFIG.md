
# Configuration

You can configure gofastlogging either programmatically (recommended for Go) or via configuration file.

## Configuration File

Supported file types: **JSON**, **YAML**, **XML**  
File name: `fastlogging.<EXT>` (e.g. `fastlogging.json`, `fastlogging.yaml`, `fastlogging.xml`)

File location:  
- Current working directory, or  
- Path specified by the environment variable `FASTLOGGING_CONFIG_FILE`

To use a config file, set the environment variable before running your Go program:

```sh
export FASTLOGGING_CONFIG_FILE=/path/to/fastlogging.json
```

Or place the config file in the working directory.

## Example Configuration Files

### JSON

```json
...existing code...

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


### YAML

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


### XML

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

## Go API Integration

You can load configuration from file or build it programmatically. Example:

```go
import logging "gofastlogging/fastlogging"

func main() {
  // Load from config file (if FASTLOGGING_CONFIG_FILE is set)
  logger := logging.Default()
  logger.Info("Logger configured from file!")

  // Or build config in Go:
  console, err := logging.ConsoleWriterConfigNew(logging.DEBUG, true)
  if err != nil {
    panic(err)
  }
  writers := []logging.WriterConfigEnum{console}
  logger2 := logging.New(logging.DEBUG, nil, writers, nil, nil)
  logger2.Info("Logger configured in Go!")
}
```

## Best Practices

- Prefer Go API for dynamic or programmatic configuration.
- Use config files for static, environment-based, or multi-language setups.
- Always check for errors when creating writer configs in Go.

