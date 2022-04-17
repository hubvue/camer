#!/usr/bin/env node
const { spawn } = require('child_process')
const { resolve } = require('path')

const executeBinary = async () => {
  const [, , ...args] = process.argv
  const options = { cwd: process.cwd(), stdio: "inherit" }

  return spawn(resolve(__dirname, './camer'), args, options)
}

executeBinary().catch((e) => console.error(e))
