const $active = document.querySelector('.active ol')
const $completed = document.querySelector('.completed ul')

async function db(event, uri, method, body) {
    let url = `http://localhost:3320/${ event }/${ uri }`
    let headers = {method: 'GET'}

    if (method) headers.method = method
    if (body) headers.body = body
    
    let re = await fetch(url, headers)
    let d = await re.text()

    return d
}

document.querySelector('form').addEventListener('submit', async e => {
    let desc = e.target.task.value
    let id = 't' + Date.now().toString(32)
    let d = await db('data', 'todo/desc-' + id, 'POST', desc)

    if (d != '') {
        $active.innerHTML += 
`<li id="${ id }">
    <label>
        <input type="checkbox" onclick="completed(this)" data="${ id }" />
        ${ desc }
    </label>
    <b onclick="del('${ id }')">x</b>
</li>`
    }
})

fetch('http://localhost:3320/list/todo/15/desc-').then(r=>r.text()).then(d => {
    if (d == '') return

    $active.innerHTML = ''
    $completed.innerHTML = ''

    d.split('\n').forEach(async uri => {
        let id = uri.replace('/data/todo/desc-', '')
        let ok = await db('data', 'todo/ok-' + id)

        if (ok == '') {
            let desc = await db('data', 'todo/desc-' + id)

            $active.innerHTML += 
`<li id="${ id }">
    <label>
        <input type="checkbox" onclick="completed(this)" data="${ id }" />
        ${ desc }
    </label>
    <b onclick="del('${ id }')">x</b>
</li>`

        } else {
            let desc = await db('data', 'todo/desc-' + id)

            $completed.innerHTML += 
`<li id="${ id }">
    <label>
        <input type="checkbox" onclick="completed(this)" data="${ id }" />
        ${ desc }
    </label>
    <b onclick="del('${ id }')">x</b>
</li>`
        }
    })
})

async function del(id) {
    let li = document.querySelector('#' + id)
    li.parentNode.removeChild(li)

    await db('data', 'todo/desc-' + id, 'DELETE')
    await db('data', 'todo/ok-' + id, 'DELETE')
}

async function completed($el) {
    let id = $el.getAttribute('data')

    if ($el.checked) {
        let li = $active.querySelector('#' + id)

        $completed.appendChild(li)
        await db('data', 'todo/ok-' + id, 'POST', '1')
    } else {
        let li = $completed.querySelector('#' + id)

        $active.appendChild(li)
        await db('data', 'todo/ok-' + id, 'DELETE')
    } 
}