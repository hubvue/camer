import { resolve } from "node:path"
import { fileURLToPath } from "node:url"
import { copyFileSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs"

// import.meta.url -> __filename
const CAMER_ROOT = resolve(fileURLToPath(import.meta.url), "../..")
const PACKAGES_ROOT = resolve(CAMER_ROOT, "..")
const BIN_ROOT = resolve(PACKAGES_ROOT, "bin")
const CAMER_MNIFEST_PATH = resolve(CAMER_ROOT, "package.json")

console.log('CAMER_ROOT', CAMER_ROOT)
console.log('PACKAGES_ROOT', PACKAGES_ROOT)
console.log('BIN_ROOT', BIN_ROOT)
console.log('CAMER_MNIFEST_PATH', CAMER_MNIFEST_PATH)

const CAMER_MANIFEST = JSON.parse(readFileSync(CAMER_MNIFEST_PATH).toString("utf-8"))


console.log('CAMER_MANIFEST', CAMER_MANIFEST)

const generateNativePackage = (platform, arch) => {
  const packageName = `@cckim/camer-${platform}-${arch}`
  const packageRoot = resolve(PACKAGES_ROOT, `camer-${platform}-${arch}`)

  // Remove the directory just in case it already exists
  // recursive: 递归删除
  // force: 不存在则忽略
  rmSync(packageRoot, { recursive: true, force: true })

  // Create the pacakge directory
  console.log(`Create directory ${packageRoot}`)
  mkdirSync(packageRoot)

  // Generate the package.json maifest
  const { version } = CAMER_MANIFEST

  const manifest = JSON.stringify({
    name: packageName,
    version,
    os: [platform],
    cpu: [arch]
  })

  const manifestPath = resolve(packageRoot, "package.json")
  console.log(`Create manifest ${manifestPath}`)
  writeFileSync(manifestPath, manifest)

  // Copy the CLI binary
  const ext = platform === "win32" ? ".exe" : ""
  const binarySource = resolve(BIN_ROOT, `camer-${platform}-${arch}${ext}`)
  const binaryTarget = resolve(packageRoot, `camer${ext}`)

  console.log(`Copy binary ${binaryTarget}`)
  copyFileSync(binarySource, binaryTarget)
}

const writeManifest = (packageName) => {
  const manifestPath = resolve(PACKAGES_ROOT, packageName, "package.json")
  
  const manifestData = JSON.parse(
		readFileSync(manifestPath).toString("utf-8"),
	);

  const nativePackages = PLATFORMS.flatMap((platform) =>
    ARCHITECTURES.map((arch) => [
      `@cckim/camer-${platform}-${arch}`,
      CAMER_MANIFEST.version
    ])
  )
  console.log('nativePackages', nativePackages)

  manifestData["version"] = CAMER_MANIFEST.version
  manifestData["optionalDependencies"] = Object.fromEntries(nativePackages)

  console.log(`Update manifest ${manifestPath}`)
  const content = JSON.stringify(manifestData, undefined, 2)
  writeFileSync(manifestPath, content)
}



const PLATFORMS = ["win32", "darwin", "linux"]
const ARCHITECTURES = ["x64", "arm64"]

// generate native packages
for (const platform of PLATFORMS) {
  for (const arch of ARCHITECTURES) {
    generateNativePackage(platform, arch)
  }
}

writeManifest("camer")

