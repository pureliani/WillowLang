const t = () => {
    return <T>(v: T): T => {
        return v
    }
}


let x = t()<number>(1)