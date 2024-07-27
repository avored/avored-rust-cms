const FeaturesSection = () => {
    return (
        <>
            <div className="text-4xl text-primary-700 pl-5 font-semibold heading-font mt-10">
                KEY FEATURES
            </div>
            <div className="my-8">
                <div className="md:flex flex-row">
                    <div className="md:w-1/3 w-full hover:bg-primary-600 rounded-lg hover:text-white p-6 mt-3">
                        <div className="font-semibold heading-font text-xl">
                            Content Modeling
                        </div>
                        <div className="text-left text-sm mt-3">
                            A flexible and customizable content modeling system that allows
                            users to define content structures (schemas) and relationships
                            between different content types. This enables the creation of
                            structured content that can be reused across various platforms and
                            channels.
                        </div>
                    </div>
                    <div className="md:w-1/3 w-full hover:bg-primary-600 rounded-lg hover:text-white p-6 mt-3">
                        <div className="font-semibold heading-font text-xl">
                            API-first Approach
                        </div>
                        <div className="text-left text-sm mt-3">
                            Built with an API-first architecture, allowing content to be
                            accessed and delivered via APIs (RESTful or GraphQL). This
                            decoupled approach enables content to be distributed to any device
                            or platform, facilitating omnichannel content delivery.
                        </div>
                    </div>
                    <div className="md:w-1/3 w-full hover:bg-primary-600 rounded-lg hover:text-white p-6 mt-3">
                        <div className="font-semibold heading-font text-xl">
                            Content Versioning and History
                        </div>
                        <div className="text-left text-sm mt-3">
                            Support for content versioning and revision history, allowing
                            users to track changes, revert to previous versions, and
                            collaborate effectively on content creation and updates.
                        </div>
                    </div>
                </div>
            </div>
            <div className="mt-8 flex">
                <div className="md:flex flex-row">
                    <div className="md:w-1/3 w-full hover:bg-primary-600 rounded-lg hover:text-white p-6 mt-3">
                        <div className="font-semibold heading-font text-xl">
                            Scalability and Performance
                        </div>
                        <div className="text-left text-sm mt-3">
                            Designed to handle large volumes of content and high traffic
                            efficiently, with features such as caching, CDN (Content Delivery
                            Network) support, and scalability options to ensure optimal
                            performance across diverse environments.
                        </div>
                    </div>
                    <div className="md:w-1/3 w-full hover:bg-primary-600 rounded-lg hover:text-white p-6 mt-3">
                        <div className="font-semibold heading-font heading-font text-xl">
                            Integration Capabilities
                        </div>
                        <div className="text-left text-sm mt-3">
                            Extensive integration capabilities with third-party services,
                            tools, and frameworks through webhooks, plugins, or custom
                            integrations. This allows seamless connectivity with other systems
                            such as eCommerce platforms, CRM systems, analytics tools, and
                            more.
                        </div>
                    </div>
                    <div className="md:w-1/3 w-full hover:bg-primary-600 rounded-lg hover:text-white p-6 mt-3">
                        <div className="font-semibold heading-font text-xl">
                            Content Localization and Internationalization
                        </div>
                        <div className="text-left text-sm mt-3">
                            Capabilities to manage multilingual and localized content
                            efficiently, including tools for translating content, managing
                            language variations, and adapting content for different regions or
                            markets.
                        </div>
                    </div>
                </div>
            </div>
        </>
    );
};
export default FeaturesSection;
