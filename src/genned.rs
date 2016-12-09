// This code is automatically generated
//
use std::ops::Deref;
use std::borrow::Cow;

#[derive(Debug, PartialEq, Clone)]
pub struct Url<'a>(Cow<'a, [u8]>);
impl<'a> From<&'a str> for Url<'a> {
    fn from(value: &'a str) -> Url<'a> {
        Url(value.as_bytes().into())
    }
}
impl<'a> From<String> for Url<'a> {
    fn from(value: String) -> Url<'a> {
        Url(Cow::Owned(value.into()))
    }
}
impl<'a> Deref for Url<'a> {
    type Target = Cow<'a, [u8]>;
    fn deref(&self) -> &Cow<'a, [u8]> {
        &self.0
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Body<'a>(Cow<'a, [u8]>);
impl<'a> From<Vec<u8>> for Body<'a> {
    fn from(value: Vec<u8>) -> Body<'a> {
        Body(value.into())
    }
}
impl<'a> From<&'a [u8]> for Body<'a> {
    fn from(value: &'a [u8]) -> Body<'a> {
        Body(value.into())
    }
}
impl<'a> From<&'a str> for Body<'a> {
    fn from(value: &'a str) -> Body<'a> {
        Body(value.as_bytes().into())
    }
}
impl<'a> From<String> for Body<'a> {
    fn from(value: String) -> Body<'a> {
        Body(Cow::Owned(value.into()))
    }
}
impl<'a> Deref for Body<'a> {
    type Target = Cow<'a, [u8]>;
    fn deref(&self) -> &Cow<'a, [u8]> {
        &self.0
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct HttpRequest<'a> {
    pub url: Cow<'a, Url<'a>>,
    pub method: HttpMethod,
    pub body: Option<Cow<'a, Body<'a>>>,
}
#[derive(Debug, PartialEq, Clone)]
pub enum HttpMethod {
    Head,
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesCloseUrlParams<'a> {
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesCloseRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesCloseRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesCloseRequestParams<'a> {
        IndicesCloseRequestParams { url: IndicesCloseUrlParams::Index(index.into()).url() }
    }
}
impl<'a> IndicesCloseUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesCloseUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(8usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_close");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesCloseRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesCloseRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DeleteScriptUrlParams<'a> {
    LangId(Lang<'a>, Id<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct DeleteScriptRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> DeleteScriptRequestParams<'a> {
    pub fn lang_id<ILang: Into<Lang<'a>>, IId: Into<Id<'a>>>(lang: ILang,
                                                             id: IId)
                                                             -> DeleteScriptRequestParams<'a> {
        DeleteScriptRequestParams {
            url: DeleteScriptUrlParams::LangId(lang.into(), id.into()).url(),
        }
    }
}
impl<'a> DeleteScriptUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            DeleteScriptUrlParams::LangId(ref lang, ref id) => {
                let mut url = String::with_capacity(11usize + lang.len() + id.len());
                url.push_str("/_scripts/");
                url.push_str(lang.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a DeleteScriptRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Delete,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for DeleteScriptRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Delete,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TermvectorsUrlParams<'a> {
    IndexType(Index<'a>, Type<'a>),
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct TermvectorsRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> TermvectorsRequestParams<'a> {
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> TermvectorsRequestParams<'a> {
        TermvectorsRequestParams {
            url: TermvectorsUrlParams::IndexType(index.into(), ty.into()).url(),
            body: body.into(),
        }
    }
    pub fn index_ty_id<IIndex: Into<Index<'a>>,
                       IType: Into<Type<'a>>,
                       IId: Into<Id<'a>>,
                       IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         id: IId,
         body: IBody)
         -> TermvectorsRequestParams<'a> {
        TermvectorsRequestParams {
            url: TermvectorsUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> TermvectorsUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            TermvectorsUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(15usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_termvectors");
                Url::from(url)
            }
            TermvectorsUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                let mut url = String::with_capacity(16usize + index.len() + ty.len() + id.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                url.push_str("/_termvectors");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a TermvectorsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for TermvectorsRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum FieldStatsUrlParams<'a> {
    None,
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct FieldStatsRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> FieldStatsRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> FieldStatsRequestParams<'a> {
        FieldStatsRequestParams {
            url: FieldStatsUrlParams::None.url(),
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> FieldStatsRequestParams<'a> {
        FieldStatsRequestParams {
            url: FieldStatsUrlParams::Index(index.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> FieldStatsUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            FieldStatsUrlParams::None => Url::from("/_field_stats"),
            FieldStatsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(14usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_field_stats");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a FieldStatsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for FieldStatsRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CatThreadPoolUrlParams<'a> {
    None,
    ThreadPoolPatterns(ThreadPoolPatterns<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct CatThreadPoolRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> CatThreadPoolRequestParams<'a> {
    pub fn new() -> CatThreadPoolRequestParams<'a> {
        CatThreadPoolRequestParams { url: CatThreadPoolUrlParams::None.url() }
    }
    pub fn thread_pool_patterns<IThreadPoolPatterns: Into<ThreadPoolPatterns<'a>>>
        (thread_pool_patterns: IThreadPoolPatterns)
         -> CatThreadPoolRequestParams<'a> {
        CatThreadPoolRequestParams {
            url: CatThreadPoolUrlParams::ThreadPoolPatterns(thread_pool_patterns.into()).url(),
        }
    }
}
impl<'a> CatThreadPoolUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            CatThreadPoolUrlParams::None => Url::from("/_cat/thread_pool"),
            CatThreadPoolUrlParams::ThreadPoolPatterns(ref thread_pool_patterns) => {
                let mut url = String::with_capacity(18usize + thread_pool_patterns.len());
                url.push_str("/_cat/thread_pool/");
                url.push_str(thread_pool_patterns.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatThreadPoolRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CatThreadPoolRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SnapshotDeleteUrlParams<'a> {
    RepositorySnapshot(Repository<'a>, Snapshot<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct SnapshotDeleteRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> SnapshotDeleteRequestParams<'a> {
    pub fn repository_snapshot<IRepository: Into<Repository<'a>>, ISnapshot: Into<Snapshot<'a>>>
        (repository: IRepository,
         snapshot: ISnapshot)
         -> SnapshotDeleteRequestParams<'a> {
        SnapshotDeleteRequestParams {
            url: SnapshotDeleteUrlParams::RepositorySnapshot(repository.into(), snapshot.into())
                .url(),
        }
    }
}
impl<'a> SnapshotDeleteUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            SnapshotDeleteUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
                let mut url = String::with_capacity(12usize + repository.len() + snapshot.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                url.push_str("/");
                url.push_str(snapshot.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SnapshotDeleteRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Delete,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for SnapshotDeleteRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Delete,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesGetSettingsUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexName(Index<'a>, Name<'a>),
    Name(Name<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesGetSettingsRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesGetSettingsRequestParams<'a> {
    pub fn new() -> IndicesGetSettingsRequestParams<'a> {
        IndicesGetSettingsRequestParams { url: IndicesGetSettingsUrlParams::None.url() }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesGetSettingsRequestParams<'a> {
        IndicesGetSettingsRequestParams {
            url: IndicesGetSettingsUrlParams::Index(index.into()).url(),
        }
    }
    pub fn index_name<IIndex: Into<Index<'a>>, IName: Into<Name<'a>>>
        (index: IIndex,
         name: IName)
         -> IndicesGetSettingsRequestParams<'a> {
        IndicesGetSettingsRequestParams {
            url: IndicesGetSettingsUrlParams::IndexName(index.into(), name.into()).url(),
        }
    }
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> IndicesGetSettingsRequestParams<'a> {
        IndicesGetSettingsRequestParams {
            url: IndicesGetSettingsUrlParams::Name(name.into()).url(),
        }
    }
}
impl<'a> IndicesGetSettingsUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesGetSettingsUrlParams::None => Url::from("/_settings"),
            IndicesGetSettingsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(11usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_settings");
                Url::from(url)
            }
            IndicesGetSettingsUrlParams::IndexName(ref index, ref name) => {
                let mut url = String::with_capacity(12usize + index.len() + name.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_settings/");
                url.push_str(name.as_ref());
                Url::from(url)
            }
            IndicesGetSettingsUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(11usize + name.len());
                url.push_str("/_settings/");
                url.push_str(name.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesGetSettingsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesGetSettingsRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CreateUrlParams<'a> {
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct CreateRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> CreateRequestParams<'a> {
    pub fn index_ty_id<IIndex: Into<Index<'a>>,
                       IType: Into<Type<'a>>,
                       IId: Into<Id<'a>>,
                       IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         id: IId,
         body: IBody)
         -> CreateRequestParams<'a> {
        CreateRequestParams {
            url: CreateUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> CreateUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            CreateUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                let mut url = String::with_capacity(11usize + index.len() + ty.len() + id.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                url.push_str("/_create");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CreateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CreateRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SnapshotDeleteRepositoryUrlParams<'a> {
    Repository(Repository<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct SnapshotDeleteRepositoryRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> SnapshotDeleteRepositoryRequestParams<'a> {
    pub fn repository<IRepository: Into<Repository<'a>>>
        (repository: IRepository)
         -> SnapshotDeleteRepositoryRequestParams<'a> {
        SnapshotDeleteRepositoryRequestParams {
            url: SnapshotDeleteRepositoryUrlParams::Repository(repository.into()).url(),
        }
    }
}
impl<'a> SnapshotDeleteRepositoryUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            SnapshotDeleteRepositoryUrlParams::Repository(ref repository) => {
                let mut url = String::with_capacity(11usize + repository.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SnapshotDeleteRepositoryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Delete,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for SnapshotDeleteRepositoryRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Delete,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ClusterAllocationExplainUrlParams {
    None,
}
#[derive(Debug, PartialEq, Clone)]
pub struct ClusterAllocationExplainRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> ClusterAllocationExplainRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> ClusterAllocationExplainRequestParams<'a> {
        ClusterAllocationExplainRequestParams {
            url: ClusterAllocationExplainUrlParams::None.url(),
            body: body.into(),
        }
    }
}
impl ClusterAllocationExplainUrlParams {
    pub fn url<'a>(self) -> Url<'a> {
        match self {
            ClusterAllocationExplainUrlParams::None => Url::from("/_cluster/allocation/explain"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ClusterAllocationExplainRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for ClusterAllocationExplainRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesPutTemplateUrlParams<'a> {
    Name(Name<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesPutTemplateRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> IndicesPutTemplateRequestParams<'a> {
    pub fn name<IName: Into<Name<'a>>, IBody: Into<Body<'a>>>
        (name: IName,
         body: IBody)
         -> IndicesPutTemplateRequestParams<'a> {
        IndicesPutTemplateRequestParams {
            url: IndicesPutTemplateUrlParams::Name(name.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> IndicesPutTemplateUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesPutTemplateUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(11usize + name.len());
                url.push_str("/_template/");
                url.push_str(name.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesPutTemplateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesPutTemplateRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesGetTemplateUrlParams<'a> {
    None,
    Name(Name<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesGetTemplateRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesGetTemplateRequestParams<'a> {
    pub fn new() -> IndicesGetTemplateRequestParams<'a> {
        IndicesGetTemplateRequestParams { url: IndicesGetTemplateUrlParams::None.url() }
    }
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> IndicesGetTemplateRequestParams<'a> {
        IndicesGetTemplateRequestParams {
            url: IndicesGetTemplateUrlParams::Name(name.into()).url(),
        }
    }
}
impl<'a> IndicesGetTemplateUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesGetTemplateUrlParams::None => Url::from("/_template"),
            IndicesGetTemplateUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(11usize + name.len());
                url.push_str("/_template/");
                url.push_str(name.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesGetTemplateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesGetTemplateRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ClusterStateUrlParams<'a> {
    None,
    Metric(Metric<'a>),
    MetricIndex(Metric<'a>, Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct ClusterStateRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> ClusterStateRequestParams<'a> {
    pub fn new() -> ClusterStateRequestParams<'a> {
        ClusterStateRequestParams { url: ClusterStateUrlParams::None.url() }
    }
    pub fn metric<IMetric: Into<Metric<'a>>>(metric: IMetric) -> ClusterStateRequestParams<'a> {
        ClusterStateRequestParams { url: ClusterStateUrlParams::Metric(metric.into()).url() }
    }
    pub fn metric_index<IMetric: Into<Metric<'a>>, IIndex: Into<Index<'a>>>
        (metric: IMetric,
         index: IIndex)
         -> ClusterStateRequestParams<'a> {
        ClusterStateRequestParams {
            url: ClusterStateUrlParams::MetricIndex(metric.into(), index.into()).url(),
        }
    }
}
impl<'a> ClusterStateUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            ClusterStateUrlParams::None => Url::from("/_cluster/state"),
            ClusterStateUrlParams::Metric(ref metric) => {
                let mut url = String::with_capacity(16usize + metric.len());
                url.push_str("/_cluster/state/");
                url.push_str(metric.as_ref());
                Url::from(url)
            }
            ClusterStateUrlParams::MetricIndex(ref metric, ref index) => {
                let mut url = String::with_capacity(17usize + metric.len() + index.len());
                url.push_str("/_cluster/state/");
                url.push_str(metric.as_ref());
                url.push_str("/");
                url.push_str(index.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ClusterStateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for ClusterStateRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MsearchTemplateUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct MsearchTemplateRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> MsearchTemplateRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> MsearchTemplateRequestParams<'a> {
        MsearchTemplateRequestParams {
            url: MsearchTemplateUrlParams::None.url(),
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> MsearchTemplateRequestParams<'a> {
        MsearchTemplateRequestParams {
            url: MsearchTemplateUrlParams::Index(index.into()).url(),
            body: body.into(),
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> MsearchTemplateRequestParams<'a> {
        MsearchTemplateRequestParams {
            url: MsearchTemplateUrlParams::IndexType(index.into(), ty.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> MsearchTemplateUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            MsearchTemplateUrlParams::None => Url::from("/_msearch/template"),
            MsearchTemplateUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(19usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_msearch/template");
                Url::from(url)
            }
            MsearchTemplateUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(20usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_msearch/template");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a MsearchTemplateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for MsearchTemplateRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BulkUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct BulkRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> BulkRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> BulkRequestParams<'a> {
        BulkRequestParams {
            url: BulkUrlParams::None.url(),
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> BulkRequestParams<'a> {
        BulkRequestParams {
            url: BulkUrlParams::Index(index.into()).url(),
            body: body.into(),
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> BulkRequestParams<'a> {
        BulkRequestParams {
            url: BulkUrlParams::IndexType(index.into(), ty.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> BulkUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            BulkUrlParams::None => Url::from("/_bulk"),
            BulkUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(7usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_bulk");
                Url::from(url)
            }
            BulkUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(8usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_bulk");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a BulkRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for BulkRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExplainUrlParams<'a> {
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct ExplainRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> ExplainRequestParams<'a> {
    pub fn index_ty_id<IIndex: Into<Index<'a>>,
                       IType: Into<Type<'a>>,
                       IId: Into<Id<'a>>,
                       IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         id: IId,
         body: IBody)
         -> ExplainRequestParams<'a> {
        ExplainRequestParams {
            url: ExplainUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> ExplainUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            ExplainUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                let mut url = String::with_capacity(12usize + index.len() + ty.len() + id.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                url.push_str("/_explain");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ExplainRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for ExplainRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SuggestUrlParams<'a> {
    None,
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct SuggestRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> SuggestRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> SuggestRequestParams<'a> {
        SuggestRequestParams {
            url: SuggestUrlParams::None.url(),
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> SuggestRequestParams<'a> {
        SuggestRequestParams {
            url: SuggestUrlParams::Index(index.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> SuggestUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            SuggestUrlParams::None => Url::from("/_suggest"),
            SuggestUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(10usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_suggest");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SuggestRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for SuggestRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SnapshotGetRepositoryUrlParams<'a> {
    None,
    Repository(Repository<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct SnapshotGetRepositoryRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> SnapshotGetRepositoryRequestParams<'a> {
    pub fn new() -> SnapshotGetRepositoryRequestParams<'a> {
        SnapshotGetRepositoryRequestParams { url: SnapshotGetRepositoryUrlParams::None.url() }
    }
    pub fn repository<IRepository: Into<Repository<'a>>>
        (repository: IRepository)
         -> SnapshotGetRepositoryRequestParams<'a> {
        SnapshotGetRepositoryRequestParams {
            url: SnapshotGetRepositoryUrlParams::Repository(repository.into()).url(),
        }
    }
}
impl<'a> SnapshotGetRepositoryUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            SnapshotGetRepositoryUrlParams::None => Url::from("/_snapshot"),
            SnapshotGetRepositoryUrlParams::Repository(ref repository) => {
                let mut url = String::with_capacity(11usize + repository.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SnapshotGetRepositoryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for SnapshotGetRepositoryRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum RenderSearchTemplateUrlParams<'a> {
    None,
    Id(Id<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct RenderSearchTemplateRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> RenderSearchTemplateRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> RenderSearchTemplateRequestParams<'a> {
        RenderSearchTemplateRequestParams {
            url: RenderSearchTemplateUrlParams::None.url(),
            body: body.into(),
        }
    }
    pub fn id<IId: Into<Id<'a>>, IBody: Into<Body<'a>>>
        (id: IId,
         body: IBody)
         -> RenderSearchTemplateRequestParams<'a> {
        RenderSearchTemplateRequestParams {
            url: RenderSearchTemplateUrlParams::Id(id.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> RenderSearchTemplateUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            RenderSearchTemplateUrlParams::None => Url::from("/_render/template"),
            RenderSearchTemplateUrlParams::Id(ref id) => {
                let mut url = String::with_capacity(18usize + id.len());
                url.push_str("/_render/template/");
                url.push_str(id.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a RenderSearchTemplateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for RenderSearchTemplateRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesStatsUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexMetric(Index<'a>, Metric<'a>),
    Metric(Metric<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesStatsRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesStatsRequestParams<'a> {
    pub fn new() -> IndicesStatsRequestParams<'a> {
        IndicesStatsRequestParams { url: IndicesStatsUrlParams::None.url() }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesStatsRequestParams<'a> {
        IndicesStatsRequestParams { url: IndicesStatsUrlParams::Index(index.into()).url() }
    }
    pub fn index_metric<IIndex: Into<Index<'a>>, IMetric: Into<Metric<'a>>>
        (index: IIndex,
         metric: IMetric)
         -> IndicesStatsRequestParams<'a> {
        IndicesStatsRequestParams {
            url: IndicesStatsUrlParams::IndexMetric(index.into(), metric.into()).url(),
        }
    }
    pub fn metric<IMetric: Into<Metric<'a>>>(metric: IMetric) -> IndicesStatsRequestParams<'a> {
        IndicesStatsRequestParams { url: IndicesStatsUrlParams::Metric(metric.into()).url() }
    }
}
impl<'a> IndicesStatsUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesStatsUrlParams::None => Url::from("/_stats"),
            IndicesStatsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(8usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_stats");
                Url::from(url)
            }
            IndicesStatsUrlParams::IndexMetric(ref index, ref metric) => {
                let mut url = String::with_capacity(9usize + index.len() + metric.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_stats/");
                url.push_str(metric.as_ref());
                Url::from(url)
            }
            IndicesStatsUrlParams::Metric(ref metric) => {
                let mut url = String::with_capacity(8usize + metric.len());
                url.push_str("/_stats/");
                url.push_str(metric.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesStatsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesStatsRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CatRepositoriesUrlParams {
    None,
}
#[derive(Debug, PartialEq, Clone)]
pub struct CatRepositoriesRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> CatRepositoriesRequestParams<'a> {
    pub fn new() -> CatRepositoriesRequestParams<'a> {
        CatRepositoriesRequestParams { url: CatRepositoriesUrlParams::None.url() }
    }
}
impl CatRepositoriesUrlParams {
    pub fn url<'a>(self) -> Url<'a> {
        match self {
            CatRepositoriesUrlParams::None => Url::from("/_cat/repositories"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatRepositoriesRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CatRepositoriesRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesForcemergeUrlParams<'a> {
    None,
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesForcemergeRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesForcemergeRequestParams<'a> {
    pub fn new() -> IndicesForcemergeRequestParams<'a> {
        IndicesForcemergeRequestParams { url: IndicesForcemergeUrlParams::None.url() }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesForcemergeRequestParams<'a> {
        IndicesForcemergeRequestParams {
            url: IndicesForcemergeUrlParams::Index(index.into()).url(),
        }
    }
}
impl<'a> IndicesForcemergeUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesForcemergeUrlParams::None => Url::from("/_forcemerge"),
            IndicesForcemergeUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(13usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_forcemerge");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesForcemergeRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesForcemergeRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PingUrlParams {
    None,
}
#[derive(Debug, PartialEq, Clone)]
pub struct PingRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> PingRequestParams<'a> {
    pub fn new() -> PingRequestParams<'a> {
        PingRequestParams { url: PingUrlParams::None.url() }
    }
}
impl PingUrlParams {
    pub fn url<'a>(self) -> Url<'a> {
        match self {
            PingUrlParams::None => Url::from("/"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a PingRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Head,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for PingRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Head,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TasksGetUrlParams<'a> {
    TaskId(TaskId<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct TasksGetRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> TasksGetRequestParams<'a> {
    pub fn task_id<ITaskId: Into<TaskId<'a>>>(task_id: ITaskId) -> TasksGetRequestParams<'a> {
        TasksGetRequestParams { url: TasksGetUrlParams::TaskId(task_id.into()).url() }
    }
}
impl<'a> TasksGetUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            TasksGetUrlParams::TaskId(ref task_id) => {
                let mut url = String::with_capacity(8usize + task_id.len());
                url.push_str("/_tasks/");
                url.push_str(task_id.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a TasksGetRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for TasksGetRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesExistsUrlParams<'a> {
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesExistsRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesExistsRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesExistsRequestParams<'a> {
        IndicesExistsRequestParams { url: IndicesExistsUrlParams::Index(index.into()).url() }
    }
}
impl<'a> IndicesExistsUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesExistsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(1usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesExistsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Head,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesExistsRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Head,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesFlushSyncedUrlParams<'a> {
    None,
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesFlushSyncedRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesFlushSyncedRequestParams<'a> {
    pub fn new() -> IndicesFlushSyncedRequestParams<'a> {
        IndicesFlushSyncedRequestParams { url: IndicesFlushSyncedUrlParams::None.url() }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesFlushSyncedRequestParams<'a> {
        IndicesFlushSyncedRequestParams {
            url: IndicesFlushSyncedUrlParams::Index(index.into()).url(),
        }
    }
}
impl<'a> IndicesFlushSyncedUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesFlushSyncedUrlParams::None => Url::from("/_flush/synced"),
            IndicesFlushSyncedUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(15usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_flush/synced");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesFlushSyncedRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesFlushSyncedRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MsearchUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct MsearchRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> MsearchRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> MsearchRequestParams<'a> {
        MsearchRequestParams {
            url: MsearchUrlParams::None.url(),
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> MsearchRequestParams<'a> {
        MsearchRequestParams {
            url: MsearchUrlParams::Index(index.into()).url(),
            body: body.into(),
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> MsearchRequestParams<'a> {
        MsearchRequestParams {
            url: MsearchUrlParams::IndexType(index.into(), ty.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> MsearchUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            MsearchUrlParams::None => Url::from("/_msearch"),
            MsearchUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(10usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_msearch");
                Url::from(url)
            }
            MsearchUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(11usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_msearch");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a MsearchRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for MsearchRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum InfoUrlParams {
    None,
}
#[derive(Debug, PartialEq, Clone)]
pub struct InfoRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> InfoRequestParams<'a> {
    pub fn new() -> InfoRequestParams<'a> {
        InfoRequestParams { url: InfoUrlParams::None.url() }
    }
}
impl InfoUrlParams {
    pub fn url<'a>(self) -> Url<'a> {
        match self {
            InfoUrlParams::None => Url::from("/"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a InfoRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for InfoRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SearchTemplateUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct SearchTemplateRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> SearchTemplateRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> SearchTemplateRequestParams<'a> {
        SearchTemplateRequestParams {
            url: SearchTemplateUrlParams::None.url(),
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> SearchTemplateRequestParams<'a> {
        SearchTemplateRequestParams {
            url: SearchTemplateUrlParams::Index(index.into()).url(),
            body: body.into(),
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> SearchTemplateRequestParams<'a> {
        SearchTemplateRequestParams {
            url: SearchTemplateUrlParams::IndexType(index.into(), ty.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> SearchTemplateUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            SearchTemplateUrlParams::None => Url::from("/_search/template"),
            SearchTemplateUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(18usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_search/template");
                Url::from(url)
            }
            SearchTemplateUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(19usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_search/template");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SearchTemplateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for SearchTemplateRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesDeleteUrlParams<'a> {
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesDeleteRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesDeleteRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesDeleteRequestParams<'a> {
        IndicesDeleteRequestParams { url: IndicesDeleteUrlParams::Index(index.into()).url() }
    }
}
impl<'a> IndicesDeleteUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesDeleteUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(1usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesDeleteRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Delete,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesDeleteRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Delete,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DeleteByQueryUrlParams<'a> {
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct DeleteByQueryRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> DeleteByQueryRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> DeleteByQueryRequestParams<'a> {
        DeleteByQueryRequestParams {
            url: DeleteByQueryUrlParams::Index(index.into()).url(),
            body: body.into(),
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> DeleteByQueryRequestParams<'a> {
        DeleteByQueryRequestParams {
            url: DeleteByQueryUrlParams::IndexType(index.into(), ty.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> DeleteByQueryUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            DeleteByQueryUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(18usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_delete_by_query");
                Url::from(url)
            }
            DeleteByQueryUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(19usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_delete_by_query");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a DeleteByQueryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for DeleteByQueryRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DeleteTemplateUrlParams<'a> {
    Id(Id<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct DeleteTemplateRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> DeleteTemplateRequestParams<'a> {
    pub fn id<IId: Into<Id<'a>>>(id: IId) -> DeleteTemplateRequestParams<'a> {
        DeleteTemplateRequestParams { url: DeleteTemplateUrlParams::Id(id.into()).url() }
    }
}
impl<'a> DeleteTemplateUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            DeleteTemplateUrlParams::Id(ref id) => {
                let mut url = String::with_capacity(18usize + id.len());
                url.push_str("/_search/template/");
                url.push_str(id.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a DeleteTemplateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Delete,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for DeleteTemplateRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Delete,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesCreateUrlParams<'a> {
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesCreateRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> IndicesCreateRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> IndicesCreateRequestParams<'a> {
        IndicesCreateRequestParams {
            url: IndicesCreateUrlParams::Index(index.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> IndicesCreateUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesCreateUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(1usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesCreateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Put,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesCreateRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Put,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PercolateUrlParams<'a> {
    IndexType(Index<'a>, Type<'a>),
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct PercolateRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> PercolateRequestParams<'a> {
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> PercolateRequestParams<'a> {
        PercolateRequestParams {
            url: PercolateUrlParams::IndexType(index.into(), ty.into()).url(),
            body: body.into(),
        }
    }
    pub fn index_ty_id<IIndex: Into<Index<'a>>,
                       IType: Into<Type<'a>>,
                       IId: Into<Id<'a>>,
                       IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         id: IId,
         body: IBody)
         -> PercolateRequestParams<'a> {
        PercolateRequestParams {
            url: PercolateUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> PercolateUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            PercolateUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(13usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_percolate");
                Url::from(url)
            }
            PercolateUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                let mut url = String::with_capacity(14usize + index.len() + ty.len() + id.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                url.push_str("/_percolate");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a PercolateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for PercolateRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SearchUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct SearchRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> SearchRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> SearchRequestParams<'a> {
        SearchRequestParams {
            url: SearchUrlParams::None.url(),
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> SearchRequestParams<'a> {
        SearchRequestParams {
            url: SearchUrlParams::Index(index.into()).url(),
            body: body.into(),
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> SearchRequestParams<'a> {
        SearchRequestParams {
            url: SearchUrlParams::IndexType(index.into(), ty.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> SearchUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            SearchUrlParams::None => Url::from("/_search"),
            SearchUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(9usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_search");
                Url::from(url)
            }
            SearchUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(10usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_search");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SearchRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for SearchRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CatNodeattrsUrlParams {
    None,
}
#[derive(Debug, PartialEq, Clone)]
pub struct CatNodeattrsRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> CatNodeattrsRequestParams<'a> {
    pub fn new() -> CatNodeattrsRequestParams<'a> {
        CatNodeattrsRequestParams { url: CatNodeattrsUrlParams::None.url() }
    }
}
impl CatNodeattrsUrlParams {
    pub fn url<'a>(self) -> Url<'a> {
        match self {
            CatNodeattrsUrlParams::None => Url::from("/_cat/nodeattrs"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatNodeattrsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CatNodeattrsRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SnapshotVerifyRepositoryUrlParams<'a> {
    Repository(Repository<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct SnapshotVerifyRepositoryRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> SnapshotVerifyRepositoryRequestParams<'a> {
    pub fn repository<IRepository: Into<Repository<'a>>>
        (repository: IRepository)
         -> SnapshotVerifyRepositoryRequestParams<'a> {
        SnapshotVerifyRepositoryRequestParams {
            url: SnapshotVerifyRepositoryUrlParams::Repository(repository.into()).url(),
        }
    }
}
impl<'a> SnapshotVerifyRepositoryUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            SnapshotVerifyRepositoryUrlParams::Repository(ref repository) => {
                let mut url = String::with_capacity(19usize + repository.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                url.push_str("/_verify");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SnapshotVerifyRepositoryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for SnapshotVerifyRepositoryRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CountUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct CountRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> CountRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> CountRequestParams<'a> {
        CountRequestParams {
            url: CountUrlParams::None.url(),
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> CountRequestParams<'a> {
        CountRequestParams {
            url: CountUrlParams::Index(index.into()).url(),
            body: body.into(),
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> CountRequestParams<'a> {
        CountRequestParams {
            url: CountUrlParams::IndexType(index.into(), ty.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> CountUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            CountUrlParams::None => Url::from("/_count"),
            CountUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(8usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_count");
                Url::from(url)
            }
            CountUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(9usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_count");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CountRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CountRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CatAllocationUrlParams<'a> {
    None,
    NodeId(NodeId<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct CatAllocationRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> CatAllocationRequestParams<'a> {
    pub fn new() -> CatAllocationRequestParams<'a> {
        CatAllocationRequestParams { url: CatAllocationUrlParams::None.url() }
    }
    pub fn node_id<INodeId: Into<NodeId<'a>>>(node_id: INodeId) -> CatAllocationRequestParams<'a> {
        CatAllocationRequestParams { url: CatAllocationUrlParams::NodeId(node_id.into()).url() }
    }
}
impl<'a> CatAllocationUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            CatAllocationUrlParams::None => Url::from("/_cat/allocation"),
            CatAllocationUrlParams::NodeId(ref node_id) => {
                let mut url = String::with_capacity(17usize + node_id.len());
                url.push_str("/_cat/allocation/");
                url.push_str(node_id.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatAllocationRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CatAllocationRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesFlushUrlParams<'a> {
    None,
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesFlushRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesFlushRequestParams<'a> {
    pub fn new() -> IndicesFlushRequestParams<'a> {
        IndicesFlushRequestParams { url: IndicesFlushUrlParams::None.url() }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesFlushRequestParams<'a> {
        IndicesFlushRequestParams { url: IndicesFlushUrlParams::Index(index.into()).url() }
    }
}
impl<'a> IndicesFlushUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesFlushUrlParams::None => Url::from("/_flush"),
            IndicesFlushUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(8usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_flush");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesFlushRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesFlushRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesRefreshUrlParams<'a> {
    None,
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesRefreshRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesRefreshRequestParams<'a> {
    pub fn new() -> IndicesRefreshRequestParams<'a> {
        IndicesRefreshRequestParams { url: IndicesRefreshUrlParams::None.url() }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesRefreshRequestParams<'a> {
        IndicesRefreshRequestParams { url: IndicesRefreshUrlParams::Index(index.into()).url() }
    }
}
impl<'a> IndicesRefreshUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesRefreshUrlParams::None => Url::from("/_refresh"),
            IndicesRefreshUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(10usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_refresh");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesRefreshRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesRefreshRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CatHelpUrlParams {
    None,
}
#[derive(Debug, PartialEq, Clone)]
pub struct CatHelpRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> CatHelpRequestParams<'a> {
    pub fn new() -> CatHelpRequestParams<'a> {
        CatHelpRequestParams { url: CatHelpUrlParams::None.url() }
    }
}
impl CatHelpUrlParams {
    pub fn url<'a>(self) -> Url<'a> {
        match self {
            CatHelpUrlParams::None => Url::from("/_cat"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatHelpRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CatHelpRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SearchShardsUrlParams<'a> {
    None,
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct SearchShardsRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> SearchShardsRequestParams<'a> {
    pub fn new() -> SearchShardsRequestParams<'a> {
        SearchShardsRequestParams { url: SearchShardsUrlParams::None.url() }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> SearchShardsRequestParams<'a> {
        SearchShardsRequestParams { url: SearchShardsUrlParams::Index(index.into()).url() }
    }
}
impl<'a> SearchShardsUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            SearchShardsUrlParams::None => Url::from("/_search_shards"),
            SearchShardsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(16usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_search_shards");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SearchShardsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for SearchShardsRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ClusterHealthUrlParams<'a> {
    None,
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct ClusterHealthRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> ClusterHealthRequestParams<'a> {
    pub fn new() -> ClusterHealthRequestParams<'a> {
        ClusterHealthRequestParams { url: ClusterHealthUrlParams::None.url() }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> ClusterHealthRequestParams<'a> {
        ClusterHealthRequestParams { url: ClusterHealthUrlParams::Index(index.into()).url() }
    }
}
impl<'a> ClusterHealthUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            ClusterHealthUrlParams::None => Url::from("/_cluster/health"),
            ClusterHealthUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(17usize + index.len());
                url.push_str("/_cluster/health/");
                url.push_str(index.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ClusterHealthRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for ClusterHealthRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesExistsAliasUrlParams<'a> {
    Index(Index<'a>),
    IndexName(Index<'a>, Name<'a>),
    Name(Name<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesExistsAliasRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesExistsAliasRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesExistsAliasRequestParams<'a> {
        IndicesExistsAliasRequestParams {
            url: IndicesExistsAliasUrlParams::Index(index.into()).url(),
        }
    }
    pub fn index_name<IIndex: Into<Index<'a>>, IName: Into<Name<'a>>>
        (index: IIndex,
         name: IName)
         -> IndicesExistsAliasRequestParams<'a> {
        IndicesExistsAliasRequestParams {
            url: IndicesExistsAliasUrlParams::IndexName(index.into(), name.into()).url(),
        }
    }
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> IndicesExistsAliasRequestParams<'a> {
        IndicesExistsAliasRequestParams {
            url: IndicesExistsAliasUrlParams::Name(name.into()).url(),
        }
    }
}
impl<'a> IndicesExistsAliasUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesExistsAliasUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(8usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_alias");
                Url::from(url)
            }
            IndicesExistsAliasUrlParams::IndexName(ref index, ref name) => {
                let mut url = String::with_capacity(9usize + index.len() + name.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_alias/");
                url.push_str(name.as_ref());
                Url::from(url)
            }
            IndicesExistsAliasUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(8usize + name.len());
                url.push_str("/_alias/");
                url.push_str(name.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesExistsAliasRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Head,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesExistsAliasRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Head,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesGetFieldMappingUrlParams<'a> {
    Fields(Fields<'a>),
    IndexFields(Index<'a>, Fields<'a>),
    IndexTypeFields(Index<'a>, Type<'a>, Fields<'a>),
    TypeFields(Type<'a>, Fields<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesGetFieldMappingRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesGetFieldMappingRequestParams<'a> {
    pub fn fields<IFields: Into<Fields<'a>>>(fields: IFields)
                                             -> IndicesGetFieldMappingRequestParams<'a> {
        IndicesGetFieldMappingRequestParams {
            url: IndicesGetFieldMappingUrlParams::Fields(fields.into()).url(),
        }
    }
    pub fn index_fields<IIndex: Into<Index<'a>>, IFields: Into<Fields<'a>>>
        (index: IIndex,
         fields: IFields)
         -> IndicesGetFieldMappingRequestParams<'a> {
        IndicesGetFieldMappingRequestParams {
            url: IndicesGetFieldMappingUrlParams::IndexFields(index.into(), fields.into()).url(),
        }
    }
    pub fn index_ty_fields<IIndex: Into<Index<'a>>,
                           IType: Into<Type<'a>>,
                           IFields: Into<Fields<'a>>>
        (index: IIndex,
         ty: IType,
         fields: IFields)
         -> IndicesGetFieldMappingRequestParams<'a> {
        IndicesGetFieldMappingRequestParams {
            url: IndicesGetFieldMappingUrlParams::IndexTypeFields(index.into(),
                                                                  ty.into(),
                                                                  fields.into())
                .url(),
        }
    }
    pub fn ty_fields<IType: Into<Type<'a>>, IFields: Into<Fields<'a>>>
        (ty: IType,
         fields: IFields)
         -> IndicesGetFieldMappingRequestParams<'a> {
        IndicesGetFieldMappingRequestParams {
            url: IndicesGetFieldMappingUrlParams::TypeFields(ty.into(), fields.into()).url(),
        }
    }
}
impl<'a> IndicesGetFieldMappingUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesGetFieldMappingUrlParams::Fields(ref fields) => {
                let mut url = String::with_capacity(16usize + fields.len());
                url.push_str("/_mapping/field/");
                url.push_str(fields.as_ref());
                Url::from(url)
            }
            IndicesGetFieldMappingUrlParams::IndexFields(ref index, ref fields) => {
                let mut url = String::with_capacity(17usize + index.len() + fields.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_mapping/field/");
                url.push_str(fields.as_ref());
                Url::from(url)
            }
            IndicesGetFieldMappingUrlParams::IndexTypeFields(ref index, ref ty, ref fields) => {
                let mut url = String::with_capacity(18usize + index.len() + ty.len() +
                                                    fields.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_mapping/");
                url.push_str(ty.as_ref());
                url.push_str("/field/");
                url.push_str(fields.as_ref());
                Url::from(url)
            }
            IndicesGetFieldMappingUrlParams::TypeFields(ref ty, ref fields) => {
                let mut url = String::with_capacity(17usize + ty.len() + fields.len());
                url.push_str("/_mapping/");
                url.push_str(ty.as_ref());
                url.push_str("/field/");
                url.push_str(fields.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesGetFieldMappingRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesGetFieldMappingRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IngestPutPipelineUrlParams<'a> {
    Id(Id<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IngestPutPipelineRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> IngestPutPipelineRequestParams<'a> {
    pub fn id<IId: Into<Id<'a>>, IBody: Into<Body<'a>>>(id: IId,
                                                        body: IBody)
                                                        -> IngestPutPipelineRequestParams<'a> {
        IngestPutPipelineRequestParams {
            url: IngestPutPipelineUrlParams::Id(id.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> IngestPutPipelineUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IngestPutPipelineUrlParams::Id(ref id) => {
                let mut url = String::with_capacity(18usize + id.len());
                url.push_str("/_ingest/pipeline/");
                url.push_str(id.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IngestPutPipelineRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Put,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IngestPutPipelineRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Put,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ClusterPendingTasksUrlParams {
    None,
}
#[derive(Debug, PartialEq, Clone)]
pub struct ClusterPendingTasksRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> ClusterPendingTasksRequestParams<'a> {
    pub fn new() -> ClusterPendingTasksRequestParams<'a> {
        ClusterPendingTasksRequestParams { url: ClusterPendingTasksUrlParams::None.url() }
    }
}
impl ClusterPendingTasksUrlParams {
    pub fn url<'a>(self) -> Url<'a> {
        match self {
            ClusterPendingTasksUrlParams::None => Url::from("/_cluster/pending_tasks"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ClusterPendingTasksRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for ClusterPendingTasksRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IngestSimulateUrlParams<'a> {
    None,
    Id(Id<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IngestSimulateRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> IngestSimulateRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> IngestSimulateRequestParams<'a> {
        IngestSimulateRequestParams {
            url: IngestSimulateUrlParams::None.url(),
            body: body.into(),
        }
    }
    pub fn id<IId: Into<Id<'a>>, IBody: Into<Body<'a>>>(id: IId,
                                                        body: IBody)
                                                        -> IngestSimulateRequestParams<'a> {
        IngestSimulateRequestParams {
            url: IngestSimulateUrlParams::Id(id.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> IngestSimulateUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IngestSimulateUrlParams::None => Url::from("/_ingest/pipeline/_simulate"),
            IngestSimulateUrlParams::Id(ref id) => {
                let mut url = String::with_capacity(28usize + id.len());
                url.push_str("/_ingest/pipeline/");
                url.push_str(id.as_ref());
                url.push_str("/_simulate");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IngestSimulateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IngestSimulateRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesGetAliasUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexName(Index<'a>, Name<'a>),
    Name(Name<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesGetAliasRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesGetAliasRequestParams<'a> {
    pub fn new() -> IndicesGetAliasRequestParams<'a> {
        IndicesGetAliasRequestParams { url: IndicesGetAliasUrlParams::None.url() }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesGetAliasRequestParams<'a> {
        IndicesGetAliasRequestParams { url: IndicesGetAliasUrlParams::Index(index.into()).url() }
    }
    pub fn index_name<IIndex: Into<Index<'a>>, IName: Into<Name<'a>>>
        (index: IIndex,
         name: IName)
         -> IndicesGetAliasRequestParams<'a> {
        IndicesGetAliasRequestParams {
            url: IndicesGetAliasUrlParams::IndexName(index.into(), name.into()).url(),
        }
    }
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> IndicesGetAliasRequestParams<'a> {
        IndicesGetAliasRequestParams { url: IndicesGetAliasUrlParams::Name(name.into()).url() }
    }
}
impl<'a> IndicesGetAliasUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesGetAliasUrlParams::None => Url::from("/_alias"),
            IndicesGetAliasUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(8usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_alias");
                Url::from(url)
            }
            IndicesGetAliasUrlParams::IndexName(ref index, ref name) => {
                let mut url = String::with_capacity(9usize + index.len() + name.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_alias/");
                url.push_str(name.as_ref());
                Url::from(url)
            }
            IndicesGetAliasUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(8usize + name.len());
                url.push_str("/_alias/");
                url.push_str(name.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesGetAliasRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesGetAliasRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum GetScriptUrlParams<'a> {
    LangId(Lang<'a>, Id<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct GetScriptRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> GetScriptRequestParams<'a> {
    pub fn lang_id<ILang: Into<Lang<'a>>, IId: Into<Id<'a>>>(lang: ILang,
                                                             id: IId)
                                                             -> GetScriptRequestParams<'a> {
        GetScriptRequestParams { url: GetScriptUrlParams::LangId(lang.into(), id.into()).url() }
    }
}
impl<'a> GetScriptUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            GetScriptUrlParams::LangId(ref lang, ref id) => {
                let mut url = String::with_capacity(11usize + lang.len() + id.len());
                url.push_str("/_scripts/");
                url.push_str(lang.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a GetScriptRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for GetScriptRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesRecoveryUrlParams<'a> {
    None,
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesRecoveryRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesRecoveryRequestParams<'a> {
    pub fn new() -> IndicesRecoveryRequestParams<'a> {
        IndicesRecoveryRequestParams { url: IndicesRecoveryUrlParams::None.url() }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesRecoveryRequestParams<'a> {
        IndicesRecoveryRequestParams { url: IndicesRecoveryUrlParams::Index(index.into()).url() }
    }
}
impl<'a> IndicesRecoveryUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesRecoveryUrlParams::None => Url::from("/_recovery"),
            IndicesRecoveryUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(11usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_recovery");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesRecoveryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesRecoveryRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IngestDeletePipelineUrlParams<'a> {
    Id(Id<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IngestDeletePipelineRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IngestDeletePipelineRequestParams<'a> {
    pub fn id<IId: Into<Id<'a>>>(id: IId) -> IngestDeletePipelineRequestParams<'a> {
        IngestDeletePipelineRequestParams {
            url: IngestDeletePipelineUrlParams::Id(id.into()).url(),
        }
    }
}
impl<'a> IngestDeletePipelineUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IngestDeletePipelineUrlParams::Id(ref id) => {
                let mut url = String::with_capacity(18usize + id.len());
                url.push_str("/_ingest/pipeline/");
                url.push_str(id.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IngestDeletePipelineRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Delete,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IngestDeletePipelineRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Delete,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TasksCancelUrlParams<'a> {
    None,
    TaskId(TaskId<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct TasksCancelRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> TasksCancelRequestParams<'a> {
    pub fn new() -> TasksCancelRequestParams<'a> {
        TasksCancelRequestParams { url: TasksCancelUrlParams::None.url() }
    }
    pub fn task_id<ITaskId: Into<TaskId<'a>>>(task_id: ITaskId) -> TasksCancelRequestParams<'a> {
        TasksCancelRequestParams { url: TasksCancelUrlParams::TaskId(task_id.into()).url() }
    }
}
impl<'a> TasksCancelUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            TasksCancelUrlParams::None => Url::from("/_tasks/_cancel"),
            TasksCancelUrlParams::TaskId(ref task_id) => {
                let mut url = String::with_capacity(16usize + task_id.len());
                url.push_str("/_tasks/");
                url.push_str(task_id.as_ref());
                url.push_str("/_cancel");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a TasksCancelRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for TasksCancelRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesClearCacheUrlParams<'a> {
    None,
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesClearCacheRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesClearCacheRequestParams<'a> {
    pub fn new() -> IndicesClearCacheRequestParams<'a> {
        IndicesClearCacheRequestParams { url: IndicesClearCacheUrlParams::None.url() }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesClearCacheRequestParams<'a> {
        IndicesClearCacheRequestParams {
            url: IndicesClearCacheUrlParams::Index(index.into()).url(),
        }
    }
}
impl<'a> IndicesClearCacheUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesClearCacheUrlParams::None => Url::from("/_cache/clear"),
            IndicesClearCacheUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(14usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_cache/clear");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesClearCacheRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesClearCacheRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DeleteUrlParams<'a> {
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct DeleteRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> DeleteRequestParams<'a> {
    pub fn index_ty_id<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IId: Into<Id<'a>>>
        (index: IIndex,
         ty: IType,
         id: IId)
         -> DeleteRequestParams<'a> {
        DeleteRequestParams {
            url: DeleteUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
        }
    }
}
impl<'a> DeleteUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            DeleteUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                let mut url = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a DeleteRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Delete,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for DeleteRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Delete,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesPutMappingUrlParams<'a> {
    IndexType(Index<'a>, Type<'a>),
    Type(Type<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesPutMappingRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> IndicesPutMappingRequestParams<'a> {
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> IndicesPutMappingRequestParams<'a> {
        IndicesPutMappingRequestParams {
            url: IndicesPutMappingUrlParams::IndexType(index.into(), ty.into()).url(),
            body: body.into(),
        }
    }
    pub fn ty<IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (ty: IType,
         body: IBody)
         -> IndicesPutMappingRequestParams<'a> {
        IndicesPutMappingRequestParams {
            url: IndicesPutMappingUrlParams::Type(ty.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> IndicesPutMappingUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesPutMappingUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(12usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_mappings/");
                url.push_str(ty.as_ref());
                Url::from(url)
            }
            IndicesPutMappingUrlParams::Type(ref ty) => {
                let mut url = String::with_capacity(11usize + ty.len());
                url.push_str("/_mappings/");
                url.push_str(ty.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesPutMappingRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesPutMappingRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CatAliasesUrlParams<'a> {
    None,
    Name(Name<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct CatAliasesRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> CatAliasesRequestParams<'a> {
    pub fn new() -> CatAliasesRequestParams<'a> {
        CatAliasesRequestParams { url: CatAliasesUrlParams::None.url() }
    }
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> CatAliasesRequestParams<'a> {
        CatAliasesRequestParams { url: CatAliasesUrlParams::Name(name.into()).url() }
    }
}
impl<'a> CatAliasesUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            CatAliasesUrlParams::None => Url::from("/_cat/aliases"),
            CatAliasesUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(14usize + name.len());
                url.push_str("/_cat/aliases/");
                url.push_str(name.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatAliasesRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CatAliasesRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ClusterStatsUrlParams<'a> {
    None,
    NodeId(NodeId<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct ClusterStatsRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> ClusterStatsRequestParams<'a> {
    pub fn new() -> ClusterStatsRequestParams<'a> {
        ClusterStatsRequestParams { url: ClusterStatsUrlParams::None.url() }
    }
    pub fn node_id<INodeId: Into<NodeId<'a>>>(node_id: INodeId) -> ClusterStatsRequestParams<'a> {
        ClusterStatsRequestParams { url: ClusterStatsUrlParams::NodeId(node_id.into()).url() }
    }
}
impl<'a> ClusterStatsUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            ClusterStatsUrlParams::None => Url::from("/_cluster/stats"),
            ClusterStatsUrlParams::NodeId(ref node_id) => {
                let mut url = String::with_capacity(22usize + node_id.len());
                url.push_str("/_cluster/stats/nodes/");
                url.push_str(node_id.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ClusterStatsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for ClusterStatsRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesValidateQueryUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesValidateQueryRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> IndicesValidateQueryRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> IndicesValidateQueryRequestParams<'a> {
        IndicesValidateQueryRequestParams {
            url: IndicesValidateQueryUrlParams::None.url(),
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> IndicesValidateQueryRequestParams<'a> {
        IndicesValidateQueryRequestParams {
            url: IndicesValidateQueryUrlParams::Index(index.into()).url(),
            body: body.into(),
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> IndicesValidateQueryRequestParams<'a> {
        IndicesValidateQueryRequestParams {
            url: IndicesValidateQueryUrlParams::IndexType(index.into(), ty.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> IndicesValidateQueryUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesValidateQueryUrlParams::None => Url::from("/_validate/query"),
            IndicesValidateQueryUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(17usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_validate/query");
                Url::from(url)
            }
            IndicesValidateQueryUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(18usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_validate/query");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesValidateQueryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesValidateQueryRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CatPendingTasksUrlParams {
    None,
}
#[derive(Debug, PartialEq, Clone)]
pub struct CatPendingTasksRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> CatPendingTasksRequestParams<'a> {
    pub fn new() -> CatPendingTasksRequestParams<'a> {
        CatPendingTasksRequestParams { url: CatPendingTasksUrlParams::None.url() }
    }
}
impl CatPendingTasksUrlParams {
    pub fn url<'a>(self) -> Url<'a> {
        match self {
            CatPendingTasksUrlParams::None => Url::from("/_cat/pending_tasks"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatPendingTasksRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CatPendingTasksRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ClearScrollUrlParams<'a> {
    None,
    ScrollId(ScrollId<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct ClearScrollRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> ClearScrollRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> ClearScrollRequestParams<'a> {
        ClearScrollRequestParams {
            url: ClearScrollUrlParams::None.url(),
            body: body.into(),
        }
    }
    pub fn scroll_id<IScrollId: Into<ScrollId<'a>>, IBody: Into<Body<'a>>>
        (scroll_id: IScrollId,
         body: IBody)
         -> ClearScrollRequestParams<'a> {
        ClearScrollRequestParams {
            url: ClearScrollUrlParams::ScrollId(scroll_id.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> ClearScrollUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            ClearScrollUrlParams::None => Url::from("/_search/scroll"),
            ClearScrollUrlParams::ScrollId(ref scroll_id) => {
                let mut url = String::with_capacity(16usize + scroll_id.len());
                url.push_str("/_search/scroll/");
                url.push_str(scroll_id.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ClearScrollRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Delete,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for ClearScrollRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Delete,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CatShardsUrlParams<'a> {
    None,
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct CatShardsRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> CatShardsRequestParams<'a> {
    pub fn new() -> CatShardsRequestParams<'a> {
        CatShardsRequestParams { url: CatShardsUrlParams::None.url() }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> CatShardsRequestParams<'a> {
        CatShardsRequestParams { url: CatShardsUrlParams::Index(index.into()).url() }
    }
}
impl<'a> CatShardsUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            CatShardsUrlParams::None => Url::from("/_cat/shards"),
            CatShardsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(13usize + index.len());
                url.push_str("/_cat/shards/");
                url.push_str(index.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatShardsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CatShardsRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesShardStoresUrlParams<'a> {
    None,
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesShardStoresRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesShardStoresRequestParams<'a> {
    pub fn new() -> IndicesShardStoresRequestParams<'a> {
        IndicesShardStoresRequestParams { url: IndicesShardStoresUrlParams::None.url() }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesShardStoresRequestParams<'a> {
        IndicesShardStoresRequestParams {
            url: IndicesShardStoresUrlParams::Index(index.into()).url(),
        }
    }
}
impl<'a> IndicesShardStoresUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesShardStoresUrlParams::None => Url::from("/_shard_stores"),
            IndicesShardStoresUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(15usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_shard_stores");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesShardStoresRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesShardStoresRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesUpdateAliasesUrlParams {
    None,
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesUpdateAliasesRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> IndicesUpdateAliasesRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> IndicesUpdateAliasesRequestParams<'a> {
        IndicesUpdateAliasesRequestParams {
            url: IndicesUpdateAliasesUrlParams::None.url(),
            body: body.into(),
        }
    }
}
impl IndicesUpdateAliasesUrlParams {
    pub fn url<'a>(self) -> Url<'a> {
        match self {
            IndicesUpdateAliasesUrlParams::None => Url::from("/_aliases"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesUpdateAliasesRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesUpdateAliasesRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CatSegmentsUrlParams<'a> {
    None,
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct CatSegmentsRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> CatSegmentsRequestParams<'a> {
    pub fn new() -> CatSegmentsRequestParams<'a> {
        CatSegmentsRequestParams { url: CatSegmentsUrlParams::None.url() }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> CatSegmentsRequestParams<'a> {
        CatSegmentsRequestParams { url: CatSegmentsUrlParams::Index(index.into()).url() }
    }
}
impl<'a> CatSegmentsUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            CatSegmentsUrlParams::None => Url::from("/_cat/segments"),
            CatSegmentsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(15usize + index.len());
                url.push_str("/_cat/segments/");
                url.push_str(index.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatSegmentsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CatSegmentsRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MpercolateUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct MpercolateRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> MpercolateRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> MpercolateRequestParams<'a> {
        MpercolateRequestParams {
            url: MpercolateUrlParams::None.url(),
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> MpercolateRequestParams<'a> {
        MpercolateRequestParams {
            url: MpercolateUrlParams::Index(index.into()).url(),
            body: body.into(),
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> MpercolateRequestParams<'a> {
        MpercolateRequestParams {
            url: MpercolateUrlParams::IndexType(index.into(), ty.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> MpercolateUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            MpercolateUrlParams::None => Url::from("/_mpercolate"),
            MpercolateUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(13usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_mpercolate");
                Url::from(url)
            }
            MpercolateUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(14usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_mpercolate");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a MpercolateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for MpercolateRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesOpenUrlParams<'a> {
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesOpenRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesOpenRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesOpenRequestParams<'a> {
        IndicesOpenRequestParams { url: IndicesOpenUrlParams::Index(index.into()).url() }
    }
}
impl<'a> IndicesOpenUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesOpenUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(7usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_open");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesOpenRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesOpenRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum GetUrlParams<'a> {
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct GetRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> GetRequestParams<'a> {
    pub fn index_ty_id<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IId: Into<Id<'a>>>
        (index: IIndex,
         ty: IType,
         id: IId)
         -> GetRequestParams<'a> {
        GetRequestParams {
            url: GetUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
        }
    }
}
impl<'a> GetUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            GetUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                let mut url = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a GetRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for GetRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum UpdateByQueryUrlParams<'a> {
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct UpdateByQueryRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> UpdateByQueryRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> UpdateByQueryRequestParams<'a> {
        UpdateByQueryRequestParams {
            url: UpdateByQueryUrlParams::Index(index.into()).url(),
            body: body.into(),
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> UpdateByQueryRequestParams<'a> {
        UpdateByQueryRequestParams {
            url: UpdateByQueryUrlParams::IndexType(index.into(), ty.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> UpdateByQueryUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            UpdateByQueryUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(18usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_update_by_query");
                Url::from(url)
            }
            UpdateByQueryUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(19usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_update_by_query");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a UpdateByQueryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for UpdateByQueryRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MtermvectorsUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct MtermvectorsRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> MtermvectorsRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> MtermvectorsRequestParams<'a> {
        MtermvectorsRequestParams {
            url: MtermvectorsUrlParams::None.url(),
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> MtermvectorsRequestParams<'a> {
        MtermvectorsRequestParams {
            url: MtermvectorsUrlParams::Index(index.into()).url(),
            body: body.into(),
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> MtermvectorsRequestParams<'a> {
        MtermvectorsRequestParams {
            url: MtermvectorsUrlParams::IndexType(index.into(), ty.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> MtermvectorsUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            MtermvectorsUrlParams::None => Url::from("/_mtermvectors"),
            MtermvectorsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(15usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_mtermvectors");
                Url::from(url)
            }
            MtermvectorsUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(16usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_mtermvectors");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a MtermvectorsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for MtermvectorsRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CatRecoveryUrlParams<'a> {
    None,
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct CatRecoveryRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> CatRecoveryRequestParams<'a> {
    pub fn new() -> CatRecoveryRequestParams<'a> {
        CatRecoveryRequestParams { url: CatRecoveryUrlParams::None.url() }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> CatRecoveryRequestParams<'a> {
        CatRecoveryRequestParams { url: CatRecoveryUrlParams::Index(index.into()).url() }
    }
}
impl<'a> CatRecoveryUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            CatRecoveryUrlParams::None => Url::from("/_cat/recovery"),
            CatRecoveryUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(15usize + index.len());
                url.push_str("/_cat/recovery/");
                url.push_str(index.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatRecoveryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CatRecoveryRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SnapshotRestoreUrlParams<'a> {
    RepositorySnapshot(Repository<'a>, Snapshot<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct SnapshotRestoreRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> SnapshotRestoreRequestParams<'a> {
    pub fn repository_snapshot<IRepository: Into<Repository<'a>>,
                               ISnapshot: Into<Snapshot<'a>>,
                               IBody: Into<Body<'a>>>
        (repository: IRepository,
         snapshot: ISnapshot,
         body: IBody)
         -> SnapshotRestoreRequestParams<'a> {
        SnapshotRestoreRequestParams {
            url: SnapshotRestoreUrlParams::RepositorySnapshot(repository.into(), snapshot.into())
                .url(),
            body: body.into(),
        }
    }
}
impl<'a> SnapshotRestoreUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            SnapshotRestoreUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
                let mut url = String::with_capacity(21usize + repository.len() + snapshot.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                url.push_str("/");
                url.push_str(snapshot.as_ref());
                url.push_str("/_restore");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SnapshotRestoreRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for SnapshotRestoreRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReindexUrlParams {
    None,
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReindexRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> ReindexRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> ReindexRequestParams<'a> {
        ReindexRequestParams {
            url: ReindexUrlParams::None.url(),
            body: body.into(),
        }
    }
}
impl ReindexUrlParams {
    pub fn url<'a>(self) -> Url<'a> {
        match self {
            ReindexUrlParams::None => Url::from("/_reindex"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ReindexRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for ReindexRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CatHealthUrlParams {
    None,
}
#[derive(Debug, PartialEq, Clone)]
pub struct CatHealthRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> CatHealthRequestParams<'a> {
    pub fn new() -> CatHealthRequestParams<'a> {
        CatHealthRequestParams { url: CatHealthUrlParams::None.url() }
    }
}
impl CatHealthUrlParams {
    pub fn url<'a>(self) -> Url<'a> {
        match self {
            CatHealthUrlParams::None => Url::from("/_cat/health"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatHealthRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CatHealthRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CatCountUrlParams<'a> {
    None,
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct CatCountRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> CatCountRequestParams<'a> {
    pub fn new() -> CatCountRequestParams<'a> {
        CatCountRequestParams { url: CatCountUrlParams::None.url() }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> CatCountRequestParams<'a> {
        CatCountRequestParams { url: CatCountUrlParams::Index(index.into()).url() }
    }
}
impl<'a> CatCountUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            CatCountUrlParams::None => Url::from("/_cat/count"),
            CatCountUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(12usize + index.len());
                url.push_str("/_cat/count/");
                url.push_str(index.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatCountRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CatCountRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CatSnapshotsUrlParams<'a> {
    None,
    Repository(Repository<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct CatSnapshotsRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> CatSnapshotsRequestParams<'a> {
    pub fn new() -> CatSnapshotsRequestParams<'a> {
        CatSnapshotsRequestParams { url: CatSnapshotsUrlParams::None.url() }
    }
    pub fn repository<IRepository: Into<Repository<'a>>>(repository: IRepository)
                                                         -> CatSnapshotsRequestParams<'a> {
        CatSnapshotsRequestParams {
            url: CatSnapshotsUrlParams::Repository(repository.into()).url(),
        }
    }
}
impl<'a> CatSnapshotsUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            CatSnapshotsUrlParams::None => Url::from("/_cat/snapshots"),
            CatSnapshotsUrlParams::Repository(ref repository) => {
                let mut url = String::with_capacity(16usize + repository.len());
                url.push_str("/_cat/snapshots/");
                url.push_str(repository.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatSnapshotsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CatSnapshotsRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesGetMappingUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
    Type(Type<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesGetMappingRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesGetMappingRequestParams<'a> {
    pub fn new() -> IndicesGetMappingRequestParams<'a> {
        IndicesGetMappingRequestParams { url: IndicesGetMappingUrlParams::None.url() }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesGetMappingRequestParams<'a> {
        IndicesGetMappingRequestParams {
            url: IndicesGetMappingUrlParams::Index(index.into()).url(),
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>>
        (index: IIndex,
         ty: IType)
         -> IndicesGetMappingRequestParams<'a> {
        IndicesGetMappingRequestParams {
            url: IndicesGetMappingUrlParams::IndexType(index.into(), ty.into()).url(),
        }
    }
    pub fn ty<IType: Into<Type<'a>>>(ty: IType) -> IndicesGetMappingRequestParams<'a> {
        IndicesGetMappingRequestParams { url: IndicesGetMappingUrlParams::Type(ty.into()).url() }
    }
}
impl<'a> IndicesGetMappingUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesGetMappingUrlParams::None => Url::from("/_mapping"),
            IndicesGetMappingUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(10usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_mapping");
                Url::from(url)
            }
            IndicesGetMappingUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(11usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_mapping/");
                url.push_str(ty.as_ref());
                Url::from(url)
            }
            IndicesGetMappingUrlParams::Type(ref ty) => {
                let mut url = String::with_capacity(10usize + ty.len());
                url.push_str("/_mapping/");
                url.push_str(ty.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesGetMappingRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesGetMappingRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SnapshotGetUrlParams<'a> {
    RepositorySnapshot(Repository<'a>, Snapshot<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct SnapshotGetRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> SnapshotGetRequestParams<'a> {
    pub fn repository_snapshot<IRepository: Into<Repository<'a>>, ISnapshot: Into<Snapshot<'a>>>
        (repository: IRepository,
         snapshot: ISnapshot)
         -> SnapshotGetRequestParams<'a> {
        SnapshotGetRequestParams {
            url: SnapshotGetUrlParams::RepositorySnapshot(repository.into(), snapshot.into()).url(),
        }
    }
}
impl<'a> SnapshotGetUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            SnapshotGetUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
                let mut url = String::with_capacity(12usize + repository.len() + snapshot.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                url.push_str("/");
                url.push_str(snapshot.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SnapshotGetRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for SnapshotGetRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CatNodesUrlParams {
    None,
}
#[derive(Debug, PartialEq, Clone)]
pub struct CatNodesRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> CatNodesRequestParams<'a> {
    pub fn new() -> CatNodesRequestParams<'a> {
        CatNodesRequestParams { url: CatNodesUrlParams::None.url() }
    }
}
impl CatNodesUrlParams {
    pub fn url<'a>(self) -> Url<'a> {
        match self {
            CatNodesUrlParams::None => Url::from("/_cat/nodes"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatNodesRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CatNodesRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExistsUrlParams<'a> {
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct ExistsRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> ExistsRequestParams<'a> {
    pub fn index_ty_id<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IId: Into<Id<'a>>>
        (index: IIndex,
         ty: IType,
         id: IId)
         -> ExistsRequestParams<'a> {
        ExistsRequestParams {
            url: ExistsUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
        }
    }
}
impl<'a> ExistsUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            ExistsUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                let mut url = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ExistsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Head,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for ExistsRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Head,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ClusterRerouteUrlParams {
    None,
}
#[derive(Debug, PartialEq, Clone)]
pub struct ClusterRerouteRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> ClusterRerouteRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> ClusterRerouteRequestParams<'a> {
        ClusterRerouteRequestParams {
            url: ClusterRerouteUrlParams::None.url(),
            body: body.into(),
        }
    }
}
impl ClusterRerouteUrlParams {
    pub fn url<'a>(self) -> Url<'a> {
        match self {
            ClusterRerouteUrlParams::None => Url::from("/_cluster/reroute"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ClusterRerouteRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for ClusterRerouteRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum NodesHotThreadsUrlParams<'a> {
    None,
    NodeId(NodeId<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct NodesHotThreadsRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> NodesHotThreadsRequestParams<'a> {
    pub fn new() -> NodesHotThreadsRequestParams<'a> {
        NodesHotThreadsRequestParams { url: NodesHotThreadsUrlParams::None.url() }
    }
    pub fn node_id<INodeId: Into<NodeId<'a>>>(node_id: INodeId)
                                              -> NodesHotThreadsRequestParams<'a> {
        NodesHotThreadsRequestParams { url: NodesHotThreadsUrlParams::NodeId(node_id.into()).url() }
    }
}
impl<'a> NodesHotThreadsUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            NodesHotThreadsUrlParams::None => Url::from("/_nodes/hot_threads"),
            NodesHotThreadsUrlParams::NodeId(ref node_id) => {
                let mut url = String::with_capacity(20usize + node_id.len());
                url.push_str("/_nodes/");
                url.push_str(node_id.as_ref());
                url.push_str("/hot_threads");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a NodesHotThreadsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for NodesHotThreadsRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum NodesStatsUrlParams<'a> {
    None,
    Metric(Metric<'a>),
    MetricIndexMetric(Metric<'a>, IndexMetric<'a>),
    NodeId(NodeId<'a>),
    NodeIdMetric(NodeId<'a>, Metric<'a>),
    NodeIdMetricIndexMetric(NodeId<'a>, Metric<'a>, IndexMetric<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct NodesStatsRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> NodesStatsRequestParams<'a> {
    pub fn new() -> NodesStatsRequestParams<'a> {
        NodesStatsRequestParams { url: NodesStatsUrlParams::None.url() }
    }
    pub fn metric<IMetric: Into<Metric<'a>>>(metric: IMetric) -> NodesStatsRequestParams<'a> {
        NodesStatsRequestParams { url: NodesStatsUrlParams::Metric(metric.into()).url() }
    }
    pub fn metric_index_metric<IMetric: Into<Metric<'a>>, IIndexMetric: Into<IndexMetric<'a>>>
        (metric: IMetric,
         index_metric: IIndexMetric)
         -> NodesStatsRequestParams<'a> {
        NodesStatsRequestParams {
            url: NodesStatsUrlParams::MetricIndexMetric(metric.into(), index_metric.into()).url(),
        }
    }
    pub fn node_id<INodeId: Into<NodeId<'a>>>(node_id: INodeId) -> NodesStatsRequestParams<'a> {
        NodesStatsRequestParams { url: NodesStatsUrlParams::NodeId(node_id.into()).url() }
    }
    pub fn node_id_metric<INodeId: Into<NodeId<'a>>, IMetric: Into<Metric<'a>>>
        (node_id: INodeId,
         metric: IMetric)
         -> NodesStatsRequestParams<'a> {
        NodesStatsRequestParams {
            url: NodesStatsUrlParams::NodeIdMetric(node_id.into(), metric.into()).url(),
        }
    }
    pub fn node_id_metric_index_metric<INodeId: Into<NodeId<'a>>,
                                       IMetric: Into<Metric<'a>>,
                                       IIndexMetric: Into<IndexMetric<'a>>>
        (node_id: INodeId,
         metric: IMetric,
         index_metric: IIndexMetric)
         -> NodesStatsRequestParams<'a> {
        NodesStatsRequestParams {
            url: NodesStatsUrlParams::NodeIdMetricIndexMetric(node_id.into(),
                                                              metric.into(),
                                                              index_metric.into())
                .url(),
        }
    }
}
impl<'a> NodesStatsUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            NodesStatsUrlParams::None => Url::from("/_nodes/stats"),
            NodesStatsUrlParams::Metric(ref metric) => {
                let mut url = String::with_capacity(14usize + metric.len());
                url.push_str("/_nodes/stats/");
                url.push_str(metric.as_ref());
                Url::from(url)
            }
            NodesStatsUrlParams::MetricIndexMetric(ref metric, ref index_metric) => {
                let mut url = String::with_capacity(15usize + metric.len() + index_metric.len());
                url.push_str("/_nodes/stats/");
                url.push_str(metric.as_ref());
                url.push_str("/");
                url.push_str(index_metric.as_ref());
                Url::from(url)
            }
            NodesStatsUrlParams::NodeId(ref node_id) => {
                let mut url = String::with_capacity(14usize + node_id.len());
                url.push_str("/_nodes/");
                url.push_str(node_id.as_ref());
                url.push_str("/stats");
                Url::from(url)
            }
            NodesStatsUrlParams::NodeIdMetric(ref node_id, ref metric) => {
                let mut url = String::with_capacity(15usize + node_id.len() + metric.len());
                url.push_str("/_nodes/");
                url.push_str(node_id.as_ref());
                url.push_str("/stats/");
                url.push_str(metric.as_ref());
                Url::from(url)
            }
            NodesStatsUrlParams::NodeIdMetricIndexMetric(ref node_id,
                                                         ref metric,
                                                         ref index_metric) => {
                let mut url = String::with_capacity(16usize + node_id.len() + metric.len() +
                                                    index_metric.len());
                url.push_str("/_nodes/");
                url.push_str(node_id.as_ref());
                url.push_str("/stats/");
                url.push_str(metric.as_ref());
                url.push_str("/");
                url.push_str(index_metric.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a NodesStatsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for NodesStatsRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IngestGetPipelineUrlParams<'a> {
    None,
    Id(Id<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IngestGetPipelineRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IngestGetPipelineRequestParams<'a> {
    pub fn new() -> IngestGetPipelineRequestParams<'a> {
        IngestGetPipelineRequestParams { url: IngestGetPipelineUrlParams::None.url() }
    }
    pub fn id<IId: Into<Id<'a>>>(id: IId) -> IngestGetPipelineRequestParams<'a> {
        IngestGetPipelineRequestParams { url: IngestGetPipelineUrlParams::Id(id.into()).url() }
    }
}
impl<'a> IngestGetPipelineUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IngestGetPipelineUrlParams::None => Url::from("/_ingest/pipeline"),
            IngestGetPipelineUrlParams::Id(ref id) => {
                let mut url = String::with_capacity(18usize + id.len());
                url.push_str("/_ingest/pipeline/");
                url.push_str(id.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IngestGetPipelineRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IngestGetPipelineRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PutTemplateUrlParams<'a> {
    Id(Id<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct PutTemplateRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> PutTemplateRequestParams<'a> {
    pub fn id<IId: Into<Id<'a>>, IBody: Into<Body<'a>>>(id: IId,
                                                        body: IBody)
                                                        -> PutTemplateRequestParams<'a> {
        PutTemplateRequestParams {
            url: PutTemplateUrlParams::Id(id.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> PutTemplateUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            PutTemplateUrlParams::Id(ref id) => {
                let mut url = String::with_capacity(18usize + id.len());
                url.push_str("/_search/template/");
                url.push_str(id.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a PutTemplateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for PutTemplateRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum GetSourceUrlParams<'a> {
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct GetSourceRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> GetSourceRequestParams<'a> {
    pub fn index_ty_id<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IId: Into<Id<'a>>>
        (index: IIndex,
         ty: IType,
         id: IId)
         -> GetSourceRequestParams<'a> {
        GetSourceRequestParams {
            url: GetSourceUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
        }
    }
}
impl<'a> GetSourceUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            GetSourceUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                let mut url = String::with_capacity(11usize + index.len() + ty.len() + id.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                url.push_str("/_source");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a GetSourceRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for GetSourceRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SnapshotCreateUrlParams<'a> {
    RepositorySnapshot(Repository<'a>, Snapshot<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct SnapshotCreateRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> SnapshotCreateRequestParams<'a> {
    pub fn repository_snapshot<IRepository: Into<Repository<'a>>,
                               ISnapshot: Into<Snapshot<'a>>,
                               IBody: Into<Body<'a>>>
        (repository: IRepository,
         snapshot: ISnapshot,
         body: IBody)
         -> SnapshotCreateRequestParams<'a> {
        SnapshotCreateRequestParams {
            url: SnapshotCreateUrlParams::RepositorySnapshot(repository.into(), snapshot.into())
                .url(),
            body: body.into(),
        }
    }
}
impl<'a> SnapshotCreateUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            SnapshotCreateUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
                let mut url = String::with_capacity(12usize + repository.len() + snapshot.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                url.push_str("/");
                url.push_str(snapshot.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SnapshotCreateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for SnapshotCreateRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ScrollUrlParams<'a> {
    None,
    ScrollId(ScrollId<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct ScrollRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> ScrollRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> ScrollRequestParams<'a> {
        ScrollRequestParams {
            url: ScrollUrlParams::None.url(),
            body: body.into(),
        }
    }
    pub fn scroll_id<IScrollId: Into<ScrollId<'a>>, IBody: Into<Body<'a>>>
        (scroll_id: IScrollId,
         body: IBody)
         -> ScrollRequestParams<'a> {
        ScrollRequestParams {
            url: ScrollUrlParams::ScrollId(scroll_id.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> ScrollUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            ScrollUrlParams::None => Url::from("/_search/scroll"),
            ScrollUrlParams::ScrollId(ref scroll_id) => {
                let mut url = String::with_capacity(16usize + scroll_id.len());
                url.push_str("/_search/scroll/");
                url.push_str(scroll_id.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ScrollRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for ScrollRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SnapshotStatusUrlParams<'a> {
    None,
    Repository(Repository<'a>),
    RepositorySnapshot(Repository<'a>, Snapshot<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct SnapshotStatusRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> SnapshotStatusRequestParams<'a> {
    pub fn new() -> SnapshotStatusRequestParams<'a> {
        SnapshotStatusRequestParams { url: SnapshotStatusUrlParams::None.url() }
    }
    pub fn repository<IRepository: Into<Repository<'a>>>(repository: IRepository)
                                                         -> SnapshotStatusRequestParams<'a> {
        SnapshotStatusRequestParams {
            url: SnapshotStatusUrlParams::Repository(repository.into()).url(),
        }
    }
    pub fn repository_snapshot<IRepository: Into<Repository<'a>>, ISnapshot: Into<Snapshot<'a>>>
        (repository: IRepository,
         snapshot: ISnapshot)
         -> SnapshotStatusRequestParams<'a> {
        SnapshotStatusRequestParams {
            url: SnapshotStatusUrlParams::RepositorySnapshot(repository.into(), snapshot.into())
                .url(),
        }
    }
}
impl<'a> SnapshotStatusUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            SnapshotStatusUrlParams::None => Url::from("/_snapshot/_status"),
            SnapshotStatusUrlParams::Repository(ref repository) => {
                let mut url = String::with_capacity(19usize + repository.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                url.push_str("/_status");
                Url::from(url)
            }
            SnapshotStatusUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
                let mut url = String::with_capacity(20usize + repository.len() + snapshot.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                url.push_str("/");
                url.push_str(snapshot.as_ref());
                url.push_str("/_status");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SnapshotStatusRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for SnapshotStatusRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MgetUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct MgetRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> MgetRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> MgetRequestParams<'a> {
        MgetRequestParams {
            url: MgetUrlParams::None.url(),
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> MgetRequestParams<'a> {
        MgetRequestParams {
            url: MgetUrlParams::Index(index.into()).url(),
            body: body.into(),
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> MgetRequestParams<'a> {
        MgetRequestParams {
            url: MgetUrlParams::IndexType(index.into(), ty.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> MgetUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            MgetUrlParams::None => Url::from("/_mget"),
            MgetUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(7usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_mget");
                Url::from(url)
            }
            MgetUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(8usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_mget");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a MgetRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for MgetRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesExistsTemplateUrlParams<'a> {
    Name(Name<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesExistsTemplateRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesExistsTemplateRequestParams<'a> {
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> IndicesExistsTemplateRequestParams<'a> {
        IndicesExistsTemplateRequestParams {
            url: IndicesExistsTemplateUrlParams::Name(name.into()).url(),
        }
    }
}
impl<'a> IndicesExistsTemplateUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesExistsTemplateUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(11usize + name.len());
                url.push_str("/_template/");
                url.push_str(name.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesExistsTemplateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Head,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesExistsTemplateRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Head,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesGetUpgradeUrlParams<'a> {
    None,
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesGetUpgradeRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesGetUpgradeRequestParams<'a> {
    pub fn new() -> IndicesGetUpgradeRequestParams<'a> {
        IndicesGetUpgradeRequestParams { url: IndicesGetUpgradeUrlParams::None.url() }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesGetUpgradeRequestParams<'a> {
        IndicesGetUpgradeRequestParams {
            url: IndicesGetUpgradeUrlParams::Index(index.into()).url(),
        }
    }
}
impl<'a> IndicesGetUpgradeUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesGetUpgradeUrlParams::None => Url::from("/_upgrade"),
            IndicesGetUpgradeUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(10usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_upgrade");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesGetUpgradeRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesGetUpgradeRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PutScriptUrlParams<'a> {
    LangId(Lang<'a>, Id<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct PutScriptRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> PutScriptRequestParams<'a> {
    pub fn lang_id<ILang: Into<Lang<'a>>, IId: Into<Id<'a>>, IBody: Into<Body<'a>>>
        (lang: ILang,
         id: IId,
         body: IBody)
         -> PutScriptRequestParams<'a> {
        PutScriptRequestParams {
            url: PutScriptUrlParams::LangId(lang.into(), id.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> PutScriptUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            PutScriptUrlParams::LangId(ref lang, ref id) => {
                let mut url = String::with_capacity(11usize + lang.len() + id.len());
                url.push_str("/_scripts/");
                url.push_str(lang.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a PutScriptRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for PutScriptRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum GetTemplateUrlParams<'a> {
    Id(Id<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct GetTemplateRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> GetTemplateRequestParams<'a> {
    pub fn id<IId: Into<Id<'a>>>(id: IId) -> GetTemplateRequestParams<'a> {
        GetTemplateRequestParams { url: GetTemplateUrlParams::Id(id.into()).url() }
    }
}
impl<'a> GetTemplateUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            GetTemplateUrlParams::Id(ref id) => {
                let mut url = String::with_capacity(18usize + id.len());
                url.push_str("/_search/template/");
                url.push_str(id.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a GetTemplateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for GetTemplateRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesDeleteTemplateUrlParams<'a> {
    Name(Name<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesDeleteTemplateRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesDeleteTemplateRequestParams<'a> {
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> IndicesDeleteTemplateRequestParams<'a> {
        IndicesDeleteTemplateRequestParams {
            url: IndicesDeleteTemplateUrlParams::Name(name.into()).url(),
        }
    }
}
impl<'a> IndicesDeleteTemplateUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesDeleteTemplateUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(11usize + name.len());
                url.push_str("/_template/");
                url.push_str(name.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesDeleteTemplateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Delete,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesDeleteTemplateRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Delete,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndexUrlParams<'a> {
    IndexType(Index<'a>, Type<'a>),
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndexRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> IndexRequestParams<'a> {
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> IndexRequestParams<'a> {
        IndexRequestParams {
            url: IndexUrlParams::IndexType(index.into(), ty.into()).url(),
            body: body.into(),
        }
    }
    pub fn index_ty_id<IIndex: Into<Index<'a>>,
                       IType: Into<Type<'a>>,
                       IId: Into<Id<'a>>,
                       IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         id: IId,
         body: IBody)
         -> IndexRequestParams<'a> {
        IndexRequestParams {
            url: IndexUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> IndexUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndexUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(2usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                Url::from(url)
            }
            IndexUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                let mut url = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndexRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndexRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesPutSettingsUrlParams<'a> {
    None,
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesPutSettingsRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> IndicesPutSettingsRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> IndicesPutSettingsRequestParams<'a> {
        IndicesPutSettingsRequestParams {
            url: IndicesPutSettingsUrlParams::None.url(),
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> IndicesPutSettingsRequestParams<'a> {
        IndicesPutSettingsRequestParams {
            url: IndicesPutSettingsUrlParams::Index(index.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> IndicesPutSettingsUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesPutSettingsUrlParams::None => Url::from("/_settings"),
            IndicesPutSettingsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(11usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_settings");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesPutSettingsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Put,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesPutSettingsRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Put,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CatTemplatesUrlParams<'a> {
    None,
    Name(Name<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct CatTemplatesRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> CatTemplatesRequestParams<'a> {
    pub fn new() -> CatTemplatesRequestParams<'a> {
        CatTemplatesRequestParams { url: CatTemplatesUrlParams::None.url() }
    }
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> CatTemplatesRequestParams<'a> {
        CatTemplatesRequestParams { url: CatTemplatesUrlParams::Name(name.into()).url() }
    }
}
impl<'a> CatTemplatesUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            CatTemplatesUrlParams::None => Url::from("/_cat/templates"),
            CatTemplatesUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(16usize + name.len());
                url.push_str("/_cat/templates/");
                url.push_str(name.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatTemplatesRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CatTemplatesRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CatIndicesUrlParams<'a> {
    None,
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct CatIndicesRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> CatIndicesRequestParams<'a> {
    pub fn new() -> CatIndicesRequestParams<'a> {
        CatIndicesRequestParams { url: CatIndicesUrlParams::None.url() }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> CatIndicesRequestParams<'a> {
        CatIndicesRequestParams { url: CatIndicesUrlParams::Index(index.into()).url() }
    }
}
impl<'a> CatIndicesUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            CatIndicesUrlParams::None => Url::from("/_cat/indices"),
            CatIndicesUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(14usize + index.len());
                url.push_str("/_cat/indices/");
                url.push_str(index.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatIndicesRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CatIndicesRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ClusterPutSettingsUrlParams {
    None,
}
#[derive(Debug, PartialEq, Clone)]
pub struct ClusterPutSettingsRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> ClusterPutSettingsRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> ClusterPutSettingsRequestParams<'a> {
        ClusterPutSettingsRequestParams {
            url: ClusterPutSettingsUrlParams::None.url(),
            body: body.into(),
        }
    }
}
impl ClusterPutSettingsUrlParams {
    pub fn url<'a>(self) -> Url<'a> {
        match self {
            ClusterPutSettingsUrlParams::None => Url::from("/_cluster/settings"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ClusterPutSettingsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Put,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for ClusterPutSettingsRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Put,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum UpdateUrlParams<'a> {
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct UpdateRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> UpdateRequestParams<'a> {
    pub fn index_ty_id<IIndex: Into<Index<'a>>,
                       IType: Into<Type<'a>>,
                       IId: Into<Id<'a>>,
                       IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         id: IId,
         body: IBody)
         -> UpdateRequestParams<'a> {
        UpdateRequestParams {
            url: UpdateUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> UpdateUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            UpdateUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                let mut url = String::with_capacity(11usize + index.len() + ty.len() + id.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                url.push_str("/_update");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a UpdateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for UpdateRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesPutAliasUrlParams<'a> {
    IndexName(Index<'a>, Name<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesPutAliasRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> IndicesPutAliasRequestParams<'a> {
    pub fn index_name<IIndex: Into<Index<'a>>, IName: Into<Name<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         name: IName,
         body: IBody)
         -> IndicesPutAliasRequestParams<'a> {
        IndicesPutAliasRequestParams {
            url: IndicesPutAliasUrlParams::IndexName(index.into(), name.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> IndicesPutAliasUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesPutAliasUrlParams::IndexName(ref index, ref name) => {
                let mut url = String::with_capacity(11usize + index.len() + name.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_aliases/");
                url.push_str(name.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesPutAliasRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesPutAliasRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CatPluginsUrlParams {
    None,
}
#[derive(Debug, PartialEq, Clone)]
pub struct CatPluginsRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> CatPluginsRequestParams<'a> {
    pub fn new() -> CatPluginsRequestParams<'a> {
        CatPluginsRequestParams { url: CatPluginsUrlParams::None.url() }
    }
}
impl CatPluginsUrlParams {
    pub fn url<'a>(self) -> Url<'a> {
        match self {
            CatPluginsUrlParams::None => Url::from("/_cat/plugins"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatPluginsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CatPluginsRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CountPercolateUrlParams<'a> {
    IndexType(Index<'a>, Type<'a>),
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct CountPercolateRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> CountPercolateRequestParams<'a> {
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> CountPercolateRequestParams<'a> {
        CountPercolateRequestParams {
            url: CountPercolateUrlParams::IndexType(index.into(), ty.into()).url(),
            body: body.into(),
        }
    }
    pub fn index_ty_id<IIndex: Into<Index<'a>>,
                       IType: Into<Type<'a>>,
                       IId: Into<Id<'a>>,
                       IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         id: IId,
         body: IBody)
         -> CountPercolateRequestParams<'a> {
        CountPercolateRequestParams {
            url: CountPercolateUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> CountPercolateUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            CountPercolateUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(19usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_percolate/count");
                Url::from(url)
            }
            CountPercolateUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                let mut url = String::with_capacity(20usize + index.len() + ty.len() + id.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                url.push_str("/_percolate/count");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CountPercolateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CountPercolateRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesUpgradeUrlParams<'a> {
    None,
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesUpgradeRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesUpgradeRequestParams<'a> {
    pub fn new() -> IndicesUpgradeRequestParams<'a> {
        IndicesUpgradeRequestParams { url: IndicesUpgradeUrlParams::None.url() }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesUpgradeRequestParams<'a> {
        IndicesUpgradeRequestParams { url: IndicesUpgradeUrlParams::Index(index.into()).url() }
    }
}
impl<'a> IndicesUpgradeUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesUpgradeUrlParams::None => Url::from("/_upgrade"),
            IndicesUpgradeUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(10usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_upgrade");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesUpgradeRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesUpgradeRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesDeleteAliasUrlParams<'a> {
    IndexName(Index<'a>, Name<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesDeleteAliasRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesDeleteAliasRequestParams<'a> {
    pub fn index_name<IIndex: Into<Index<'a>>, IName: Into<Name<'a>>>
        (index: IIndex,
         name: IName)
         -> IndicesDeleteAliasRequestParams<'a> {
        IndicesDeleteAliasRequestParams {
            url: IndicesDeleteAliasUrlParams::IndexName(index.into(), name.into()).url(),
        }
    }
}
impl<'a> IndicesDeleteAliasUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesDeleteAliasUrlParams::IndexName(ref index, ref name) => {
                let mut url = String::with_capacity(11usize + index.len() + name.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_aliases/");
                url.push_str(name.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesDeleteAliasRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Delete,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesDeleteAliasRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Delete,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CatTasksUrlParams {
    None,
}
#[derive(Debug, PartialEq, Clone)]
pub struct CatTasksRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> CatTasksRequestParams<'a> {
    pub fn new() -> CatTasksRequestParams<'a> {
        CatTasksRequestParams { url: CatTasksUrlParams::None.url() }
    }
}
impl CatTasksUrlParams {
    pub fn url<'a>(self) -> Url<'a> {
        match self {
            CatTasksUrlParams::None => Url::from("/_cat/tasks"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatTasksRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CatTasksRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesRolloverUrlParams<'a> {
    Alias(Alias<'a>),
    AliasNewIndex(Alias<'a>, NewIndex<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesRolloverRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> IndicesRolloverRequestParams<'a> {
    pub fn alias<IAlias: Into<Alias<'a>>, IBody: Into<Body<'a>>>
        (alias: IAlias,
         body: IBody)
         -> IndicesRolloverRequestParams<'a> {
        IndicesRolloverRequestParams {
            url: IndicesRolloverUrlParams::Alias(alias.into()).url(),
            body: body.into(),
        }
    }
    pub fn alias_new_index<IAlias: Into<Alias<'a>>,
                           INewIndex: Into<NewIndex<'a>>,
                           IBody: Into<Body<'a>>>
        (alias: IAlias,
         new_index: INewIndex,
         body: IBody)
         -> IndicesRolloverRequestParams<'a> {
        IndicesRolloverRequestParams {
            url: IndicesRolloverUrlParams::AliasNewIndex(alias.into(), new_index.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> IndicesRolloverUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesRolloverUrlParams::Alias(ref alias) => {
                let mut url = String::with_capacity(11usize + alias.len());
                url.push_str("/");
                url.push_str(alias.as_ref());
                url.push_str("/_rollover");
                Url::from(url)
            }
            IndicesRolloverUrlParams::AliasNewIndex(ref alias, ref new_index) => {
                let mut url = String::with_capacity(12usize + alias.len() + new_index.len());
                url.push_str("/");
                url.push_str(alias.as_ref());
                url.push_str("/_rollover/");
                url.push_str(new_index.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesRolloverRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesRolloverRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReindexRethrottleUrlParams<'a> {
    TaskId(TaskId<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReindexRethrottleRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> ReindexRethrottleRequestParams<'a> {
    pub fn task_id<ITaskId: Into<TaskId<'a>>>(task_id: ITaskId)
                                              -> ReindexRethrottleRequestParams<'a> {
        ReindexRethrottleRequestParams {
            url: ReindexRethrottleUrlParams::TaskId(task_id.into()).url(),
        }
    }
}
impl<'a> ReindexRethrottleUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            ReindexRethrottleUrlParams::TaskId(ref task_id) => {
                let mut url = String::with_capacity(30usize + task_id.len());
                url.push_str("/_delete_by_query/");
                url.push_str(task_id.as_ref());
                url.push_str("/_rethrottle");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ReindexRethrottleRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for ReindexRethrottleRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SnapshotCreateRepositoryUrlParams<'a> {
    Repository(Repository<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct SnapshotCreateRepositoryRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> SnapshotCreateRepositoryRequestParams<'a> {
    pub fn repository<IRepository: Into<Repository<'a>>, IBody: Into<Body<'a>>>
        (repository: IRepository,
         body: IBody)
         -> SnapshotCreateRepositoryRequestParams<'a> {
        SnapshotCreateRepositoryRequestParams {
            url: SnapshotCreateRepositoryUrlParams::Repository(repository.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> SnapshotCreateRepositoryUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            SnapshotCreateRepositoryUrlParams::Repository(ref repository) => {
                let mut url = String::with_capacity(11usize + repository.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SnapshotCreateRepositoryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for SnapshotCreateRepositoryRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesGetUrlParams<'a> {
    Index(Index<'a>),
    IndexFeature(Index<'a>, Feature<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesGetRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesGetRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesGetRequestParams<'a> {
        IndicesGetRequestParams { url: IndicesGetUrlParams::Index(index.into()).url() }
    }
    pub fn index_feature<IIndex: Into<Index<'a>>, IFeature: Into<Feature<'a>>>
        (index: IIndex,
         feature: IFeature)
         -> IndicesGetRequestParams<'a> {
        IndicesGetRequestParams {
            url: IndicesGetUrlParams::IndexFeature(index.into(), feature.into()).url(),
        }
    }
}
impl<'a> IndicesGetUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesGetUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(1usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                Url::from(url)
            }
            IndicesGetUrlParams::IndexFeature(ref index, ref feature) => {
                let mut url = String::with_capacity(2usize + index.len() + feature.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(feature.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesGetRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesGetRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesAnalyzeUrlParams<'a> {
    None,
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesAnalyzeRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> IndicesAnalyzeRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> IndicesAnalyzeRequestParams<'a> {
        IndicesAnalyzeRequestParams {
            url: IndicesAnalyzeUrlParams::None.url(),
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> IndicesAnalyzeRequestParams<'a> {
        IndicesAnalyzeRequestParams {
            url: IndicesAnalyzeUrlParams::Index(index.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> IndicesAnalyzeUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesAnalyzeUrlParams::None => Url::from("/_analyze"),
            IndicesAnalyzeUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(10usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_analyze");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesAnalyzeRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesAnalyzeRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CatFielddataUrlParams<'a> {
    None,
    Fields(Fields<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct CatFielddataRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> CatFielddataRequestParams<'a> {
    pub fn new() -> CatFielddataRequestParams<'a> {
        CatFielddataRequestParams { url: CatFielddataUrlParams::None.url() }
    }
    pub fn fields<IFields: Into<Fields<'a>>>(fields: IFields) -> CatFielddataRequestParams<'a> {
        CatFielddataRequestParams { url: CatFielddataUrlParams::Fields(fields.into()).url() }
    }
}
impl<'a> CatFielddataUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            CatFielddataUrlParams::None => Url::from("/_cat/fielddata"),
            CatFielddataUrlParams::Fields(ref fields) => {
                let mut url = String::with_capacity(16usize + fields.len());
                url.push_str("/_cat/fielddata/");
                url.push_str(fields.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatFielddataRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CatFielddataRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesSegmentsUrlParams<'a> {
    None,
    Index(Index<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesSegmentsRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesSegmentsRequestParams<'a> {
    pub fn new() -> IndicesSegmentsRequestParams<'a> {
        IndicesSegmentsRequestParams { url: IndicesSegmentsUrlParams::None.url() }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesSegmentsRequestParams<'a> {
        IndicesSegmentsRequestParams { url: IndicesSegmentsUrlParams::Index(index.into()).url() }
    }
}
impl<'a> IndicesSegmentsUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesSegmentsUrlParams::None => Url::from("/_segments"),
            IndicesSegmentsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(11usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_segments");
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesSegmentsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesSegmentsRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesShrinkUrlParams<'a> {
    IndexTarget(Index<'a>, Target<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesShrinkRequestParams<'a> {
    pub url: Url<'a>,
    pub body: Body<'a>,
}
impl<'a> IndicesShrinkRequestParams<'a> {
    pub fn index_target<IIndex: Into<Index<'a>>, ITarget: Into<Target<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         target: ITarget,
         body: IBody)
         -> IndicesShrinkRequestParams<'a> {
        IndicesShrinkRequestParams {
            url: IndicesShrinkUrlParams::IndexTarget(index.into(), target.into()).url(),
            body: body.into(),
        }
    }
}
impl<'a> IndicesShrinkUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesShrinkUrlParams::IndexTarget(ref index, ref target) => {
                let mut url = String::with_capacity(10usize + index.len() + target.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_shrink/");
                url.push_str(target.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesShrinkRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Borrowed(&self.body)),
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesShrinkRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Post,
            body: Some(Cow::Owned(self.body)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TasksListUrlParams {
    None,
}
#[derive(Debug, PartialEq, Clone)]
pub struct TasksListRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> TasksListRequestParams<'a> {
    pub fn new() -> TasksListRequestParams<'a> {
        TasksListRequestParams { url: TasksListUrlParams::None.url() }
    }
}
impl TasksListUrlParams {
    pub fn url<'a>(self) -> Url<'a> {
        match self {
            TasksListUrlParams::None => Url::from("/_tasks"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a TasksListRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for TasksListRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CatMasterUrlParams {
    None,
}
#[derive(Debug, PartialEq, Clone)]
pub struct CatMasterRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> CatMasterRequestParams<'a> {
    pub fn new() -> CatMasterRequestParams<'a> {
        CatMasterRequestParams { url: CatMasterUrlParams::None.url() }
    }
}
impl CatMasterUrlParams {
    pub fn url<'a>(self) -> Url<'a> {
        match self {
            CatMasterUrlParams::None => Url::from("/_cat/master"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatMasterRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for CatMasterRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndicesExistsTypeUrlParams<'a> {
    IndexType(Index<'a>, Type<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct IndicesExistsTypeRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> IndicesExistsTypeRequestParams<'a> {
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>>
        (index: IIndex,
         ty: IType)
         -> IndicesExistsTypeRequestParams<'a> {
        IndicesExistsTypeRequestParams {
            url: IndicesExistsTypeUrlParams::IndexType(index.into(), ty.into()).url(),
        }
    }
}
impl<'a> IndicesExistsTypeUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            IndicesExistsTypeUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(11usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_mapping/");
                url.push_str(ty.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesExistsTypeRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Head,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for IndicesExistsTypeRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Head,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ClusterGetSettingsUrlParams {
    None,
}
#[derive(Debug, PartialEq, Clone)]
pub struct ClusterGetSettingsRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> ClusterGetSettingsRequestParams<'a> {
    pub fn new() -> ClusterGetSettingsRequestParams<'a> {
        ClusterGetSettingsRequestParams { url: ClusterGetSettingsUrlParams::None.url() }
    }
}
impl ClusterGetSettingsUrlParams {
    pub fn url<'a>(self) -> Url<'a> {
        match self {
            ClusterGetSettingsUrlParams::None => Url::from("/_cluster/settings"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ClusterGetSettingsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for ClusterGetSettingsRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum NodesInfoUrlParams<'a> {
    None,
    Metric(Metric<'a>),
    NodeId(NodeId<'a>),
    NodeIdMetric(NodeId<'a>, Metric<'a>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct NodesInfoRequestParams<'a> {
    pub url: Url<'a>,
}
impl<'a> NodesInfoRequestParams<'a> {
    pub fn new() -> NodesInfoRequestParams<'a> {
        NodesInfoRequestParams { url: NodesInfoUrlParams::None.url() }
    }
    pub fn metric<IMetric: Into<Metric<'a>>>(metric: IMetric) -> NodesInfoRequestParams<'a> {
        NodesInfoRequestParams { url: NodesInfoUrlParams::Metric(metric.into()).url() }
    }
    pub fn node_id<INodeId: Into<NodeId<'a>>>(node_id: INodeId) -> NodesInfoRequestParams<'a> {
        NodesInfoRequestParams { url: NodesInfoUrlParams::NodeId(node_id.into()).url() }
    }
    pub fn node_id_metric<INodeId: Into<NodeId<'a>>, IMetric: Into<Metric<'a>>>
        (node_id: INodeId,
         metric: IMetric)
         -> NodesInfoRequestParams<'a> {
        NodesInfoRequestParams {
            url: NodesInfoUrlParams::NodeIdMetric(node_id.into(), metric.into()).url(),
        }
    }
}
impl<'a> NodesInfoUrlParams<'a> {
    pub fn url(self) -> Url<'a> {
        match self {
            NodesInfoUrlParams::None => Url::from("/_nodes"),
            NodesInfoUrlParams::Metric(ref metric) => {
                let mut url = String::with_capacity(8usize + metric.len());
                url.push_str("/_nodes/");
                url.push_str(metric.as_ref());
                Url::from(url)
            }
            NodesInfoUrlParams::NodeId(ref node_id) => {
                let mut url = String::with_capacity(8usize + node_id.len());
                url.push_str("/_nodes/");
                url.push_str(node_id.as_ref());
                Url::from(url)
            }
            NodesInfoUrlParams::NodeIdMetric(ref node_id, ref metric) => {
                let mut url = String::with_capacity(9usize + node_id.len() + metric.len());
                url.push_str("/_nodes/");
                url.push_str(node_id.as_ref());
                url.push_str("/");
                url.push_str(metric.as_ref());
                Url::from(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a NodesInfoRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Borrowed(&self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}
impl<'a> Into<HttpRequest<'a>> for NodesInfoRequestParams<'a> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: Cow::Owned(self.url),
            method: HttpMethod::Get,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Alias<'a>(pub Cow<'a, str>);
impl<'a> From<&'a str> for Alias<'a> {
    fn from(value: &'a str) -> Alias<'a> {
        Alias(Cow::Borrowed(value))
    }
}
impl<'a> From<String> for Alias<'a> {
    fn from(value: String) -> Alias<'a> {
        Alias(Cow::Owned(value))
    }
}
impl<'a> ::std::ops::Deref for Alias<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Feature<'a>(pub Cow<'a, str>);
impl<'a> From<&'a str> for Feature<'a> {
    fn from(value: &'a str) -> Feature<'a> {
        Feature(Cow::Borrowed(value))
    }
}
impl<'a> From<String> for Feature<'a> {
    fn from(value: String) -> Feature<'a> {
        Feature(Cow::Owned(value))
    }
}
impl<'a> ::std::ops::Deref for Feature<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Fields<'a>(pub Cow<'a, str>);
impl<'a> From<&'a str> for Fields<'a> {
    fn from(value: &'a str) -> Fields<'a> {
        Fields(Cow::Borrowed(value))
    }
}
impl<'a> From<String> for Fields<'a> {
    fn from(value: String) -> Fields<'a> {
        Fields(Cow::Owned(value))
    }
}
impl<'a> ::std::ops::Deref for Fields<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Id<'a>(pub Cow<'a, str>);
impl<'a> From<&'a str> for Id<'a> {
    fn from(value: &'a str) -> Id<'a> {
        Id(Cow::Borrowed(value))
    }
}
impl<'a> From<String> for Id<'a> {
    fn from(value: String) -> Id<'a> {
        Id(Cow::Owned(value))
    }
}
impl<'a> ::std::ops::Deref for Id<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Index<'a>(pub Cow<'a, str>);
impl<'a> From<&'a str> for Index<'a> {
    fn from(value: &'a str) -> Index<'a> {
        Index(Cow::Borrowed(value))
    }
}
impl<'a> From<String> for Index<'a> {
    fn from(value: String) -> Index<'a> {
        Index(Cow::Owned(value))
    }
}
impl<'a> ::std::ops::Deref for Index<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IndexMetric<'a>(pub Cow<'a, str>);
impl<'a> From<&'a str> for IndexMetric<'a> {
    fn from(value: &'a str) -> IndexMetric<'a> {
        IndexMetric(Cow::Borrowed(value))
    }
}
impl<'a> From<String> for IndexMetric<'a> {
    fn from(value: String) -> IndexMetric<'a> {
        IndexMetric(Cow::Owned(value))
    }
}
impl<'a> ::std::ops::Deref for IndexMetric<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Lang<'a>(pub Cow<'a, str>);
impl<'a> From<&'a str> for Lang<'a> {
    fn from(value: &'a str) -> Lang<'a> {
        Lang(Cow::Borrowed(value))
    }
}
impl<'a> From<String> for Lang<'a> {
    fn from(value: String) -> Lang<'a> {
        Lang(Cow::Owned(value))
    }
}
impl<'a> ::std::ops::Deref for Lang<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Metric<'a>(pub Cow<'a, str>);
impl<'a> From<&'a str> for Metric<'a> {
    fn from(value: &'a str) -> Metric<'a> {
        Metric(Cow::Borrowed(value))
    }
}
impl<'a> From<String> for Metric<'a> {
    fn from(value: String) -> Metric<'a> {
        Metric(Cow::Owned(value))
    }
}
impl<'a> ::std::ops::Deref for Metric<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Name<'a>(pub Cow<'a, str>);
impl<'a> From<&'a str> for Name<'a> {
    fn from(value: &'a str) -> Name<'a> {
        Name(Cow::Borrowed(value))
    }
}
impl<'a> From<String> for Name<'a> {
    fn from(value: String) -> Name<'a> {
        Name(Cow::Owned(value))
    }
}
impl<'a> ::std::ops::Deref for Name<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct NewIndex<'a>(pub Cow<'a, str>);
impl<'a> From<&'a str> for NewIndex<'a> {
    fn from(value: &'a str) -> NewIndex<'a> {
        NewIndex(Cow::Borrowed(value))
    }
}
impl<'a> From<String> for NewIndex<'a> {
    fn from(value: String) -> NewIndex<'a> {
        NewIndex(Cow::Owned(value))
    }
}
impl<'a> ::std::ops::Deref for NewIndex<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct NodeId<'a>(pub Cow<'a, str>);
impl<'a> From<&'a str> for NodeId<'a> {
    fn from(value: &'a str) -> NodeId<'a> {
        NodeId(Cow::Borrowed(value))
    }
}
impl<'a> From<String> for NodeId<'a> {
    fn from(value: String) -> NodeId<'a> {
        NodeId(Cow::Owned(value))
    }
}
impl<'a> ::std::ops::Deref for NodeId<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Repository<'a>(pub Cow<'a, str>);
impl<'a> From<&'a str> for Repository<'a> {
    fn from(value: &'a str) -> Repository<'a> {
        Repository(Cow::Borrowed(value))
    }
}
impl<'a> From<String> for Repository<'a> {
    fn from(value: String) -> Repository<'a> {
        Repository(Cow::Owned(value))
    }
}
impl<'a> ::std::ops::Deref for Repository<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScrollId<'a>(pub Cow<'a, str>);
impl<'a> From<&'a str> for ScrollId<'a> {
    fn from(value: &'a str) -> ScrollId<'a> {
        ScrollId(Cow::Borrowed(value))
    }
}
impl<'a> From<String> for ScrollId<'a> {
    fn from(value: String) -> ScrollId<'a> {
        ScrollId(Cow::Owned(value))
    }
}
impl<'a> ::std::ops::Deref for ScrollId<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Snapshot<'a>(pub Cow<'a, str>);
impl<'a> From<&'a str> for Snapshot<'a> {
    fn from(value: &'a str) -> Snapshot<'a> {
        Snapshot(Cow::Borrowed(value))
    }
}
impl<'a> From<String> for Snapshot<'a> {
    fn from(value: String) -> Snapshot<'a> {
        Snapshot(Cow::Owned(value))
    }
}
impl<'a> ::std::ops::Deref for Snapshot<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Target<'a>(pub Cow<'a, str>);
impl<'a> From<&'a str> for Target<'a> {
    fn from(value: &'a str) -> Target<'a> {
        Target(Cow::Borrowed(value))
    }
}
impl<'a> From<String> for Target<'a> {
    fn from(value: String) -> Target<'a> {
        Target(Cow::Owned(value))
    }
}
impl<'a> ::std::ops::Deref for Target<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TaskId<'a>(pub Cow<'a, str>);
impl<'a> From<&'a str> for TaskId<'a> {
    fn from(value: &'a str) -> TaskId<'a> {
        TaskId(Cow::Borrowed(value))
    }
}
impl<'a> From<String> for TaskId<'a> {
    fn from(value: String) -> TaskId<'a> {
        TaskId(Cow::Owned(value))
    }
}
impl<'a> ::std::ops::Deref for TaskId<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ThreadPoolPatterns<'a>(pub Cow<'a, str>);
impl<'a> From<&'a str> for ThreadPoolPatterns<'a> {
    fn from(value: &'a str) -> ThreadPoolPatterns<'a> {
        ThreadPoolPatterns(Cow::Borrowed(value))
    }
}
impl<'a> From<String> for ThreadPoolPatterns<'a> {
    fn from(value: String) -> ThreadPoolPatterns<'a> {
        ThreadPoolPatterns(Cow::Owned(value))
    }
}
impl<'a> ::std::ops::Deref for ThreadPoolPatterns<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Type<'a>(pub Cow<'a, str>);
impl<'a> From<&'a str> for Type<'a> {
    fn from(value: &'a str) -> Type<'a> {
        Type(Cow::Borrowed(value))
    }
}
impl<'a> From<String> for Type<'a> {
    fn from(value: String) -> Type<'a> {
        Type(Cow::Owned(value))
    }
}
impl<'a> ::std::ops::Deref for Type<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}
