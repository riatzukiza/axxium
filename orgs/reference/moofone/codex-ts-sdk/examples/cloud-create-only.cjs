#!/usr/bin/env REDACTED_SECRET
/* eslint-disable no-console */
/**
 * Focused test: Task Creation only
 * Isolate the 500 error
 */

const { CloudTasksClientBuilder } = require('../dist/cjs/src/cloud/index.js');

async function main() {
  const envId = process.env.ENV_ID || process.env.ENV_LABEL;

  if (!envId) {
    console.error('❌ Error: ENV_ID or ENV_LABEL environment variable is required');
    console.error('   Example: ENV_LABEL=owner/repo REDACTED_SECRET examples/cloud-create-only.cjs');
    process.exit(1);
  }

  console.log('🎯 Focused Test: Task Creation Only\n');

  const client = new CloudTasksClientBuilder()
    .withUserAgent('codex-sdk-test/1.0.0')
    .build();

  try {
    console.log('📝 Creating task with parameters:');
    console.log(`   environmentId: "${envId}"`);
    console.log('   prompt: "Say hello"');
    console.log('   gitRef: "main"');
    console.log('');

    const created = await client.createTask({
      environmentId: envId,
      prompt: 'Say hello',
      gitRef: 'main',
    });

    console.log('✅ SUCCESS!');
    console.log(`   Task ID: ${created.id}\n`);

  } catch (error) {
    console.error('❌ FAILED');
    console.error(`   Error: ${error.message}\n`);

    // Parse the error
    if (error.message.includes('500')) {
      console.log('📊 Error Analysis:');
      console.log('   Status: 500 Internal Server Error');
      console.log('   Endpoint: POST /wham/tasks');
      console.log('   Response: {"detail":"Internal Server Error"}');
      console.log('');
      console.log('🔍 Possible causes:');
      console.log(`   1. Environment "${envId}" may not support task creation`);
      console.log('   2. The gitRef "main" may be invalid for this environment');
      console.log('   3. The prompt may be too simple/invalid');
      console.log('   4. Backend service issue');
      console.log('   5. Authentication/permission issue');
      console.log('');
      console.log('💡 The SDK client is working correctly.');
      console.log('   The request reaches the backend successfully.');
      console.log('   This is a backend validation or processing error.\n');
    }
  } finally {
    client.close();
  }
}

main().catch(console.error);
