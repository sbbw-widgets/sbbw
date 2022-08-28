const INITIAL_SCRIPT: &str = r#"
(function() {
function Rpc() {
    const self = this;
    this._promises = {};

    this._error = (id, error) => {
        if(this._promises[id]){
            this._promises[id].reject(error);
            delete this._promises[id];
        }
    }

    this._result = (id, result) => {
        if(this._promises[id]){
            if (result.status == 200)
            try {
                this._promises[id].resolve(JSON.parse(result.data))
            } catch {
                this._promises[id].resolve(result.data)
            }
            else
                this._promises[id].reject({ code: result.status, data: result.data })
            delete this._promises[id];
        }
    }

    this.call = function(method, data) {
        let array = new Uint32Array(1);
        window.crypto.getRandomValues(array);
        const id = array[0];
        const payload = {
            method_id: id,
            method: method,
            data: JSON.stringify(data),
        };
        const promise = new Promise((resolve, reject) => {
            self._promises[id] = {resolve, reject};
        });
        window.ipc.postMessage(JSON.stringify(payload));
        return promise;
    }
}
window.external = window.external || {};
window.external.rpc = new Rpc();
window.rpc = window.external.rpc;
window.general = window.general || {};
##OTHER_VARIABLES##
})();"#;

pub fn get_initial_js() -> String {
    let base = INITIAL_SCRIPT.to_string();
    let mut others = String::new();
    others.push_str(format!("window.general.os = \"{}\";", std::env::consts::OS).as_str());
    others.push_str(format!("window.general.os_arch = \"{}\";", std::env::consts::ARCH).as_str());
    base.replace("##OTHER_VARIABLES##", &others)
}
