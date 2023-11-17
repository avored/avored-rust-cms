export default () => ({
    modal: false,
    error_message: '',
    files: [],
    init() {
        console.log("asset manager init")
    },
    close_modal() {
        this.modal = false
    },
    save_asset() {
        var formData = new FormData();
        //var imagefile = document.querySelector('#file');
        this.files.forEach((f) => formData.append('file', f));
        //formData.append("image", imagefile.files[0]);
        axios.post('/admin/store-asset', formData, {
            headers: {
                'Content-Type': 'multipart/form-data'
            }
        }).then((res) => {
            if (res.data.success === false) {
                this.error_message = "Only Jpeg and PNG file is allowed only."
                return
            }
            location.reload()
        })
    },
    file_on_change_alpine_js(e) {
        this.files.push(e.target.files[0])
    }
  });
  