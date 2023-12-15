import logo from "../../assets/logo_only.svg";

function AppHeader() {
    return (
        <header
            className="h-16 py-2 shadow-lg px-4 md:sticky top-0 bg-gray-800 z-40">
            <a href="/admin"
               className="text-white flex items-center space-x-2 group hover:text-white">
                <div>
                    <img src={logo} className="h-12"/>
                </div>

                <div>
                    <span className="text-2xl font-semibold">AvoRed</span>
                    <span className="text-xs block">Rust CMS</span>
                </div>
            </a>
        </header>
    );
}

export default AppHeader;
