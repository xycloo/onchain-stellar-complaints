'use server'

export async function ProxyPOST(request_body) {
    const res = await fetch(`${process.env.ENDPOINT}/zephyr/execute`, {
      method: 'POST',
      headers: {
        Authorization: ['Bearer', process.env.MERCURY_JWT].join(' '),
        'Content-Type': 'application/json'
      },
      body: request_body
    })
  
    const resp = await res.json();
    return resp
  }

  