export default () => ({
    modal: false,
    files: [],
    init() {
        console.log("asset manager")
    },
    close_modal() {
        this.modal = false
    },
    save_asset() {
        console.log("Test Save Asset")
        var formData = new FormData();
        //var imagefile = document.querySelector('#file');
        this.files.forEach((f) => formData.append('file', f));
        //formData.append("image", imagefile.files[0]);
        axios.post('/admin/store-asset', formData, {
            headers: {
                'Content-Type': 'multipart/form-data'
            }
        }).then((res) => {
            location.reload()
        })
    },
    file_on_change_alpine_js(e) {
        this.files.push(e.target.files[0])
    }
  });
  