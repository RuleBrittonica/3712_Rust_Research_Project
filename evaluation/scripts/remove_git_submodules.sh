for d in evaluation/testfiles/REM_Original/*; do
  [ -d "$d" ] || continue

  echo "-> Processing $d"

  if git ls-files -s -- "$d" | awk '$1==160000{found=1} END{exit !found}'; then
    git rm --cached -r --quiet -- "$d" || true
  fi

  if [ -e "$d/.git" ]; then
    rm -rf -- "$d/.git"
  fi

  m=".git/modules/$d"
  if [ -d "$m" ]; then
    rm -rf -- "$m"
  fi

  git add -- "$d"
done

git commit -m "Vendor REM_Original test repos as plain directories"
