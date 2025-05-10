
POSTS_DIR="blog/posts"
JSON_FILE="blog/posts.json"

echo "[" > $JSON_FILE

first=true
for file in $POSTS_DIR/*.md; do
  if [ -f "$file" ]; then
    filename=$(basename "$file")
    title=${filename%.md}
    
    title=$(echo $title | sed 's/-/ /g' | awk '{for(i=1;i<=NF;i++) $i=toupper(substr($i,1,1)) substr($i,2)} 1')
    
    date=$(echo $filename | grep -o "^[0-9]\{4\}-[0-9]\{2\}-[0-9]\{2\}")
    if [ -z "$date" ]; then
      date=$(date +"%Y-%m-%d")
    fi
    
    if [ "$first" = true ]; then
      first=false
    else
      echo "," >> $JSON_FILE
    fi
    
    echo "  {" >> $JSON_FILE
    echo "    \"title\": \"$title\"," >> $JSON_FILE
    echo "    \"path\": \"$file\"," >> $JSON_FILE
    echo "    \"date\": \"$date\"" >> $JSON_FILE
    echo -n "  }" >> $JSON_FILE
  fi
done

echo "" >> $JSON_FILE
echo "]" >> $JSON_FILE

echo "posts.json has been updated with $(grep -c "title" $JSON_FILE) posts."
echo "Posts JSON file is located at: $JSON_FILE"
