# QBT-Controller

A Windows system tray application, written in rust, to make quickly pausing and
resuming qBittorrent instances easy!

## Rationale

TODO

## Known Bugs

Currently, there is some instability with the `windows-rs` library causing
linkage issues with the latest windows APIs.

### Temporary Workaround

1. Compile with `cargo b`
2. Create the file `target/debug/qbt-controller.exe.manifest` with the following contents:
Create the file `` with the following contents:
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<assemblyIdentity
    version="1.0.0.0"
    processorArchitecture="*"
    name="app"
    type="win32"
/>
<dependency>
    <dependentAssembly>
        <assemblyIdentity
            type="win32"
            name="Microsoft.Windows.Common-Controls"
            version="6.0.0.0"
            processorArchitecture="*"
            publicKeyToken="6595b64144ccf1df"
            language="*"
        />
    </dependentAssembly>
</dependency>
</assembly>
```

## License

Refer to [LICENCE](./LICENSE)
