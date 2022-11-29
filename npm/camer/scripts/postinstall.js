const { platform, arch } = process

const PLATFORMS = {
  win32: {
    x64: "@cckim/camer-win32-x64/camer.exe",
    arm64: "@cckim/camer-win32-armx64/camer.exe"
  },
  darwin: {
    x64: "@cckim/camer-darwin-x64/camer",
    arm64: "@cckim/camer-darwin-arm64/camer"
  },
  linux: {
    x64: "@cckim/camer-linux-x64/camer",
    arm64: "@cckim/camer-linux-arm64/camer"
  }
}

const binName = PLATFORMS?.[platform]?.[arch]

console.log('binName', binName)
if (binName) {
  let binPath
  try {
    binPath = require.resolve(binName)
  } catch {
    console.warn(
      `The Camer CLI postinstall script failed to resolve the binary file "${binName}". Running Camer from the pm package will probably not work correctly.`
    )
  }
  
  if (binPath) {
    try {
      require("fs").chmodSync(binPath, 0o755)
    } catch {
      console.warn(
        "The Camer CLI postinstall script failed to set execution permissions to the native binary." +
        "Running Camer from the npm package will probably not work correctly."
      )
    }
  }
} else {
  console.log.warn(
    "The Camer CLI package doesn't ship with prebuilt binaries for your platform yet." +
    "You can still use the CLI by cloning the hubvue/camer repo from GitHub, " +
    "and follow the instructions there to build the CLI for your platform."
  )
}
