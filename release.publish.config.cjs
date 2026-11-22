module.exports = {
  branches: ['main'],
  tagFormat: '${version}',
  plugins: [
    '@semantic-release/commit-analyzer',
    '@semantic-release/release-notes-generator',
    [
      '@semantic-release/exec',
      {
        verifyReleaseCmd:
          './scripts/verify-release-version.sh standardizer-common ${nextRelease.version}',
        prepareCmd:
          './scripts/publish-rust-crate.sh standardizer-common ${nextRelease.version}',
      },
    ],
    [
      '@semantic-release/git',
      {
        assets: ['Cargo.toml', 'Cargo.lock', 'CHANGELOG.md'],
        message:
          'chore: release ${nextRelease.version} [skip ci]\n\n${nextRelease.notes}',
      },
    ],
    [
      '@semantic-release/github',
      {},
    ],
  ],
};
