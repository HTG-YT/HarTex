"""
SPDX-License-Identifier: AGPL-3.0-only

This file is part of HarTex.

HarTex
Copyright (c) 2021-2025 HarTex Project Developers

HarTex is free software; you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation; either version 3 of the License, or
(at your option) any later version.

HarTex is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along
with HarTex. If not, see <https://www.gnu.org/licenses/>.
"""

import argparse
import datetime
import sys
import os

from builder import HarTexBuild
from cmdrunner import run_cmd
from time import time


def bootstrap(args):
    root = os.path.abspath(os.path.join(__file__, "../../.."))
    conf = ''

    if os.path.exists(os.path.join(root, "hartex.conf")):
        with open(os.path.join(root, "hartex.conf")) as file:
            conf = file.read()
    
    build = HarTexBuild(conf=conf, args=args)

    sys.stdout.flush()
    build.build_bootstrap()
    sys.stdout.flush()

    args = [build.bootstrap_binpath()]
    args.extend(sys.argv[1:])

    env = os.environ.copy()
    run_cmd(args, is_bootstrap=True, env=env)


def parse_args(argv):
    parser = argparse.ArgumentParser(add_help=False)

    parser.add_argument("-h", "--help", action="store_true")

    return parser.parse_known_args(argv)[0]


def main():
    start = time()

    if len(sys.argv) > 1 and sys.argv[1] == "help":
        sys.argv[1] = "-h"
    
    args = parse_args(sys.argv)
    help_triggered = args.help or len(sys.argv) == 1

    if help_triggered:
        bold = "\033[1m"
        end = "\033[0m"
        print(f"{bold + 'note:' + end} Building bootstrap before processing help command.")

    status = 0
    success = "successfully"

    try:
        bootstrap(args)
    except (SystemExit, KeyboardInterrupt) as error:
        if hasattr(error, "code") and isinstance(error.code, int):
            status = error.code
        else:
            status = 1
            print(error)
        success = "unsuccessfully"

    if not help_triggered:
        print(f"Build completed {success} in {str(datetime.timedelta(seconds=int(time() - start)))}. Exit code: {status}")
    sys.exit(status)
