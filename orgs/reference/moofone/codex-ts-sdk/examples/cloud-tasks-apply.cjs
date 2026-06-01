#!/usr/bin/env REDACTED_SECRET
/* eslint-disable no-console */
/**
 * Cloud Tasks: Safe Patch Application Example
 *
 * This example demonstrates the recommended workflow for safely applying
 * cloud task diffs to your local working tree with conflict detection.
 *
 * Usage:
 *   # With mock backend (no API key needed, safe dry-run)
 *   REDACTED_SECRET examples/cloud-tasks-apply.cjs
 *
 *   # With live backend (requires API key, WILL modify files)
 *   OPENAI_API_KEY=sk-... REDACTED_SECRET examples/cloud-tasks-apply.cjs
 */

const { CloudTasksClientBuilder } = require('../dist/cjs/src/cloud/index.js');

async function waitForUserConfirmation(message) {
  return new Promise((resolve) => {
    const readline = require('readline');
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout,
    });

    rl.question(`${message} (y/N): `, (answer) => {
      rl.close();
      resolve(answer.toLowerCase() === 'y' || answer.toLowerCase() === 'yes');
    });
  });
}

async function main() {
  // Use mock backend if no API key provided
  const useMock = !process.env.OPENAI_API_KEY;
  const baseUrl = process.env.CODEX_CLOUD_TASKS_BASE_URL || 'https://chatgpt.com/backend-api';

  console.log('🔒 Cloud Tasks: Safe Patch Application\n');
  console.log(useMock ? '🔧 Using mock backend (safe, no file changes)\n' : '⚠️  Using live backend (WILL modify files)\n');

  const client = new CloudTasksClientBuilder()
    .withBaseUrl(baseUrl)
    .withBearerToken(process.env.OPENAI_API_KEY || 'mock-token')
    .withUserAgent('cloud-example-apply/0.1.0')
    .withMockBackend(useMock)
    .build();

  try {
    // Step 1: List available tasks
    console.log('📋 Fetching available tasks...');
    const tasks = await client.listTasks({ limit: 5 });

    if (tasks.length === 0) {
      console.log('ℹ️  No tasks found. Create one first!');
      console.log('\nExample:');
      console.log('  REDACTED_SECRET examples/cloud-tasks-basic.cjs');
      return;
    }

    console.log(`\nFound ${tasks.length} task(s):\n`);
    tasks.forEach((task, i) => {
      const statusIcon = task.status === 'ready' ? '✅' :
                        task.status === 'pending' ? '⏳' :
                        task.status === 'applied' ? '✔️ ' : '❌';
      console.log(`  ${i + 1}. ${statusIcon} [${task.id}] ${task.title}`);
      console.log(`     Status: ${task.status} | Files: ${task.summary.filesChanged} | +${task.summary.linesAdded} -${task.summary.linesRemoved}`);
    });

    // Step 2: Select first ready task
    const readyTask = tasks.find(t => t.status === 'ready');
    if (!readyTask) {
      console.log('\n⚠️  No ready tasks found. Wait for pending tasks to complete.');
      return;
    }

    console.log(`\n🎯 Selected task: ${readyTask.id}`);
    console.log(`   ${readyTask.title}\n`);

    // Step 3: Get the diff
    console.log('📄 Retrieving diff...');
    const diff = await client.getTaskDiff(readyTask.id);

    if (!diff) {
      console.log('⚠️  No diff available for this task');
      return;
    }

    console.log(`✅ Diff retrieved (${diff.split('\n').length} lines)\n`);

    // Show diff preview
    console.log('📝 Diff Preview (first 20 lines):');
    console.log('─'.repeat(60));
    const diffLines = diff.split('\n');
    diffLines.slice(0, 20).forEach(line => console.log(line));
    if (diffLines.length > 20) {
      console.log(`... (${diffLines.length - 20} more lines)`);
    }
    console.log('─'.repeat(60));
    console.log('');

    // Step 4: Preflight check (ALWAYS do this first)
    console.log('🔍 Running preflight check (dry-run)...');
    const preflight = await client.applyTaskPreflight(readyTask.id);

    console.log(`\nPreflight Results:`);
    console.log(`  Status: ${preflight.status}`);
    console.log(`  Message: ${preflight.message}`);
    console.log(`  Applied: ${preflight.applied ? 'Yes (would apply)' : 'No (dry-run)'}`);

    if (preflight.conflictPaths.length > 0) {
      console.log(`\n  ⚠️  CONFLICTS DETECTED:`);
      preflight.conflictPaths.forEach(path => console.log(`     - ${path}`));
      console.log('\n  ⚠️  Cannot apply patch due to conflicts!');
      console.log('  💡 Resolve conflicts manually or use a different attempt.');
      return;
    }

    if (preflight.skippedPaths.length > 0) {
      console.log(`\n  ℹ️  Files skipped (already up to date):`);
      preflight.skippedPaths.forEach(path => console.log(`     - ${path}`));
    }

    if (preflight.status !== 'success') {
      console.log(`\n  ❌ Preflight failed: ${preflight.message}`);
      return;
    }

    console.log('\n  ✅ Preflight passed! Patch can be applied cleanly.\n');

    // Step 5: Confirm before applying (if not mock)
    if (!useMock) {
      console.log('⚠️  WARNING: This will modify files in your working tree!\n');
      const confirmed = await waitForUserConfirmation('Apply patch to local files?');

      if (!confirmed) {
        console.log('\n🚫 Application cancelled by user.');
        return;
      }
      console.log('');
    }

    // Step 6: Apply the patch
    console.log('🚀 Applying patch...');
    const result = await client.applyTask(readyTask.id);

    console.log(`\n✅ Application Results:`);
    console.log(`  Status: ${result.status}`);
    console.log(`  Applied: ${result.applied ? 'Yes' : 'No'}`);
    console.log(`  Message: ${result.message}`);

    if (result.skippedPaths.length > 0) {
      console.log(`\n  ℹ️  Skipped files:`);
      result.skippedPaths.forEach(path => console.log(`     - ${path}`));
    }

    if (result.conflictPaths.length > 0) {
      console.log(`\n  ⚠️  Conflicts (unexpected!):`);
      result.conflictPaths.forEach(path => console.log(`     - ${path}`));
    }

    if (result.status === 'success') {
      console.log('\n✅ Patch applied successfully!');

      if (!useMock) {
        console.log('\n💡 Next steps:');
        console.log('  1. Review the changes: git diff');
        console.log('  2. Run tests to verify: npm test');
        console.log('  3. Commit if satisfied: git commit -am "Apply cloud task changes"');
      } else {
        console.log('\n💡 To apply for real, run with OPENAI_API_KEY set');
      }
    } else if (result.status === 'partial') {
      console.log('\n⚠️  Patch partially applied. Review conflicts.');
    } else {
      console.log('\n❌ Patch application failed.');
    }

    // Step 7: Safety tips
    console.log('\n🔒 Safety Best Practices:');
    console.log('  ✅ Always run preflight before apply');
    console.log('  ✅ Review the diff preview carefully');
    console.log('  ✅ Work on a clean git branch');
    console.log('  ✅ Test thoroughly after applying');
    console.log('  ✅ Use diffOverride to try alternate attempts if conflicts occur');

  } catch (error) {
    console.error('\n❌ Error:', error.message || error);
    if (error.code === 'UNIMPLEMENTED') {
      console.error('\n💡 Hint: Cloud tasks require codex-rs v0.45.0+');
      console.error('   Run: npm run setup with CODEX_RUST_ROOT=/path/to/codex-rs-v0.45.0');
    }
    process.exit(1);
  } finally {
    client.close();
  }
}

main().catch(console.error);
