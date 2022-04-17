#!/usr/bin/env node
const { spawn } = require('child_process')

const executeBinary = async () => {
  const [, , ...args] = process.argv
  const options = { cwd: process.cwd(), stdio: "inherit" }

  return spawn('./camer', args, options)
}

executeBinary().catch((e) => console.error(e))
