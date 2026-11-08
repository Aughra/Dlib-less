#!/bin/sh
# Checks the P1 git exercise: the human made and pushed a commit on their own.
# Usage (from anywhere): sh prof/p1-git/verifier.sh

repo="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$repo" || exit 1
ok=0

check() { # $1 = description, $2 = 0 if success
    if [ "$2" -eq 0 ]; then echo "✅ $1"; else echo "❌ $1"; ok=1; fi
}

git fetch -q origin 2>/dev/null

[ -z "$(git status --porcelain)" ]
check "rien d'oublié : aucun fichier modifié hors d'un commit" $?

git show --name-only --format= HEAD | grep -q '^apprentissage/carnet.md$'
check "le dernier commit contient le carnet" $?

! git log -1 --format=%B | grep -q 'Co-Authored-By: Claude'
check "le dernier commit est le tien (pas celui de Claude)" $?

[ "$(git rev-parse HEAD)" = "$(git rev-parse origin/main)" ]
check "le commit est poussé sur GitHub" $?

[ $ok -eq 0 ] && echo "\nBravo : exercice réussi." || echo "\nPas encore : regarde les lignes ❌."
exit $ok
