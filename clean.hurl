GET http://localhost:3000/api/public/v1/words?search={{word_inglix}}&from=inglix
accept: application/json
[Options]
variable: word_inglix=anglix
HTTP 200
[Asserts]
jsonpath "$" count > 0
[Captures]
word_id: jsonpath "$[0].id"


DELETE http://localhost:3000/api/admin/v1/words/{{word_id}}
accept: application/json
HTTP 200
