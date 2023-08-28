export async function getRequest(url: string) {
    let response = await fetch(url, {
        method: 'get',   
    }).then(response => {
        console.log("got response");
        console.log(response);
        response.json()
    });

    return response;
}
