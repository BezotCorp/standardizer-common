'use strict';

const phase = process.env.BEZOTCORP_RELEASE_PHASE;

switch (phase) {
  case 'prepare':
    module.exports = require('./release.prepare.config.cjs');
    break;

  case 'publish':
    module.exports = require('./release.publish.config.cjs');
    break;

  default:
    throw new Error(
      `Invalid release phase: ${phase ?? 'undefined'}`,
    );
}
