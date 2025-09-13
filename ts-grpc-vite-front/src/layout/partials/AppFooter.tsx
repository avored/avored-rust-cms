const AppFooter = () => {
    return (
        <div className="mt-5">
            <hr/>
            <p className="py-10 pl-3">
                &copy; AvoRed {new Date().getFullYear()}
                <a href="/privacy" className="ml-3" title="Avored privacy">
                    Privacy
                </a>
            </p>
        </div>
    );
};

export default AppFooter;
