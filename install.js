const fs = require('fs')
const path = require('path')
const os = require('os')
const { mkdirp } = require('mkdirp')

const platform = os.platform()
const arch = os.arch()

// Create a mapping from a combination of platform and architecture to the corresponding binary path.
const binaryMap = {
	'linux-aarch64': 'aarch64-unknown-linux-gnu',
	'linux-aarch64-musl': 'aarch64-unknown-linux-musl',
	'darwin-aarch64': 'aarch64-apple-darwin',
	'win32-aarch64': 'aarch64-pc-windows-msvc',
	'linux-x64': 'x86_64-unknown-linux-gnu',
	'linux-x64-musl': 'x86_64-unknown-linux-musl',
	'darwin-x64': 'x86_64-apple-darwin',
	'win32-x64': 'x86_64-pc-windows-msvc',
	'freebsd-x64': 'x86_64-unknown-freebsd',
	'darwin-universal': 'universal-apple-darwin'
}

// Convert platform and arch to a key for the binaryMap
let binaryKey = `${platform}-${arch}`
if (
	platform === 'linux' &&
	['aarch64', 'x64'].includes(arch) &&
	process.env.MUSL
) {
	binaryKey += '-musl'
}

const binaryPath = binaryMap[binaryKey]

if (!binaryPath) {
	console.error(`No precompiled binary available for ${platform}-${arch}`)
	process.exit(1)
}

const actualPath = path.join(__dirname, 'artifacts', binaryPath, 'bem.node')

// Copy the binary to the appropriate location
fs.copyFileSync(actualPath, path.join(__dirname, 'bem.node'))

const binDir = path.join(__dirname, 'node_modules', '.bin')
mkdirp.sync(binDir)

const symlinkPath = path.join(binDir, 'bem')

fs.symlink(actualPath, symlinkPath, 'file', (err) => {
	if (err) console.error('Error creating symlink:', err)
	else console.log('Symlink created')
})
