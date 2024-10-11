POST http://localhost:3000/api/admin/v1/words
accept: application/json
[Options]
variable: word_english=anglish
variable: word_inglix=anglix
{
	"english_spelling": "{{word_english}}",
	"inglix_spelling": "{{word_inglix}}"
}
HTTP 201
[Captures]
word_id: jsonpath "$"


GET http://localhost:3000/api/public/v1/words/{{word_id}}
accept: application/json
HTTP 200
[Asserts]
jsonpath "$.id" == "{{word_id}}"
jsonpath "$.english_spelling" == "{{word_english}}"
jsonpath "$.inglix_spelling" == "{{word_inglix}}"


GET http://localhost:3000/api/public/v1/words
accept: application/json
HTTP 200
[Asserts]
jsonpath "$" count > 0
jsonpath "$[*].id" includes "{{word_id}}"
jsonpath "$[*].english_spelling" includes "{{word_english}}"
jsonpath "$[*].inglix_spelling" includes "{{word_inglix}}"


GET http://localhost:3000/api/public/v1/words?search=ish&from=english
accept: application/json
HTTP 200
[Asserts]
jsonpath "$" count > 0
jsonpath "$[*].id" includes "{{word_id}}"
jsonpath "$[*].english_spelling" includes "{{word_english}}"
jsonpath "$[*].inglix_spelling" includes "{{word_inglix}}"


GET http://localhost:3000/api/public/v1/words?search=ix&from=inglix
accept: application/json
HTTP 200
[Asserts]
jsonpath "$" count > 0
jsonpath "$[*].id" includes "{{word_id}}"
jsonpath "$[*].english_spelling" includes "{{word_english}}"
jsonpath "$[*].inglix_spelling" includes "{{word_inglix}}"


GET http://localhost:3000/api/public/v1/words?search=abcdefghijklmnopqrstuvwxyz&from=english
accept: application/json
HTTP 200
[Asserts]
jsonpath "$" count == 0


GET http://localhost:3000/api/public/v1/words?search=abcdefghijklmnopqrstuvwxyz&from=invalid
accept: application/json
HTTP 400


DELETE http://localhost:3000/api/admin/v1/words/{{word_id}}
accept: application/json
HTTP 200
"{{word_id}}"
