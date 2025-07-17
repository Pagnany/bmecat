#[derive(Debug, Clone, Default)]
pub struct Article {
    pub id: String,
    pub article_details: ArtikelDetails,
    pub article_order_details: ArticleOrderDetails,
    pub article_price_details: Vec<ArticlePriceDetails>,
    pub article_feature_groups: Vec<ArticleFeatureGroup>,
    pub mime_infos: Vec<Mime>,
}

impl Article {
    pub fn get_pictures(&self) -> Vec<String> {
        let mut pictures = Vec::new();
        for mime in &self.mime_infos {
            if mime.mime_type == "image/jpeg" || mime.mime_type == "image/png" {
                pictures.push(mime.mime_source.clone());
            }
        }
        pictures
    }
}

#[derive(Debug, Clone, Default)]
pub struct ArtikelDetails {
    pub desc_short: String,
    pub desc_long: String,
    pub ean: String,
    pub supplier_alt_id: String,
    pub buyer_id: Vec<String>,
    pub manufacturer_id: String,
    pub manufacturer_name: String,
    pub manufacturer_type_desc: String,
    pub erp_group_buyer: String,
    pub erp_group_supplier: String,
    pub deliver_time: String,
    pub special_treatment_class: Vec<String>,
    pub remarks: String,
    pub segment: String,
    pub article_order: String,
    pub keywords: Vec<String>,
    pub article_staus: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ArticleFeatureGroup {
    pub sys_name: String,
    pub group_id: String,
    pub group_name: String,
    pub article_features: Vec<ArticleFeature>,
}

#[derive(Debug, Clone, Default)]
pub struct ArticleFeature {
    pub name: String,
    pub value: Vec<String>,
    pub unit: String,
    pub order: String,
    pub descr: String,
    pub value_details: String,
    pub article_variants: ArticleVariants,
}

#[derive(Debug, Clone, Default)]
pub struct ArticleVariants {
    pub article_variant: Vec<ArticleVariant>,
    pub vorder: String,
}

#[derive(Debug, Clone, Default)]
pub struct ArticleVariant {
    pub value: String,
    pub supplier_aid_supplement: String,
}

#[derive(Debug, Clone, Default)]
pub struct ArticleOrderDetails {
    pub order_unit: String,
    pub content_unit: String,
    pub no_cu_per_ou: String,
    pub price_quantity: String,
    pub quantity_min: String,
    pub quantity_interval: String,
}

#[derive(Debug, Clone, Default)]
pub struct ArticlePriceDetails {
    pub start_date: String,
    pub end_date: String,
    pub daily_price: String,
    pub article_prices: Vec<ArticlePrice>,
}

#[derive(Debug, Clone, Default)]
pub struct ArticlePrice {
    pub price_amount: String,
    pub price_currency: String,
    pub tax: String,
    pub price_factor: String,
    pub lower_bound: String,
    pub price_type: String,
    pub territory: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Mime {
    pub mime_type: String,
    pub mime_source: String,
    pub mime_descr: String,
    pub mime_alt: String,
    pub mime_purpose: String,
    pub mime_order: String,
}

#[derive(Debug, Clone, Default)]
pub struct UserDefinedExtensions {}

#[derive(Debug, Clone, Default)]
pub struct CatalogGroupSystem {
    pub group_system_id: String,
    pub group_system_name: String,
    pub catalog_structure: Vec<String>,
    pub group_system_descr: String,
}

#[derive(Debug, Clone, Default)]
pub struct CatalogStructure {
    pub group_id: String,
    pub group_name: String,
    pub group_description: String,
    pub parent_id: String,
    pub group_order: String,
    pub mime_info: Mime,
    pub user_defined_extensions: UserDefinedExtensions,
    pub keyword: String,
}

#[derive(Debug, Clone, Default)]
pub struct TNewCatalog {
    pub feature_system: Vec<FeatureSystem>,
    pub classification_system: Vec<ClassificationSystem>,
    pub catalog_group_system: CatalogGroupSystem,
    pub article: Vec<Article>,
    pub article_to_cataloggroup_map: Vec<ArticleToCatalogGroup>,
}

#[derive(Debug, Clone, Default)]
pub struct FeatureSystem {
    pub feature_system_name: String,
    pub feature_system_descr: String,
    pub feature_group: Vec<FeatureGroup>,
}

#[derive(Debug, Clone, Default)]
pub struct ClassificationSystem {}

#[derive(Debug, Clone, Default)]
pub struct ArticleToCatalogGroup {
    pub art_id: String,
    pub catalog_group_id: String,
    pub article_to_cataloggroup_map_order: String,
}

#[derive(Debug, Clone, Default)]
pub struct FeatureGroup {}
