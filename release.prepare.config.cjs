module.exports = {
  branches: ['main'],
  tagFormat: '${version}',
  plugins: [
    '@semantic-release/commit-analyzer',
    '@semantic-release/release-notes-generator',
    '@semantic-release/changelog',
    [
      '@semantic-release/exec',
      {
        prepareCmd:
          'semantic-release-cargo prepare ${nextRelease.version} && cargo update',
      },
    ],
    'semantic-release-stop-before-publish',
  ],
};
