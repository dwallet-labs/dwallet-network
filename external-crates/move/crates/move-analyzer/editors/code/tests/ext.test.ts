// Copyright (c) The Move Contributors
// SPDX-License-Identifier: BSD-3-Clause-Clear

import * as assert from 'assert';
import * as Mocha from 'mocha';
import * as vscode from 'vscode';

Mocha.suite('ext', () => {
    Mocha.test('ext_exists', () => {
        const ext = vscode.extensions.getExtension('move.move-analyzer');
        assert.ok(ext);
    });
});
