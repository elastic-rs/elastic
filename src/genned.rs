// This code is automatically generated
//
use std::marker::PhantomData;
use std::ops::Deref;
use std::borrow::Cow;

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
pub struct HttpRequest<'a> {
    pub url: Cow<'a, str>,
    pub method: HttpMethod,
    pub body: Option<&'a Body<'a>>,
}
pub enum HttpMethod {
    Head,
    Get,
    Post,
    Put,
    Delete,
}

pub enum IndicesCloseUrlParams<'a> {
    Index(Index<'a>),
}
pub struct IndicesCloseRequestParams<'a> {
    pub url_params: IndicesCloseUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesCloseRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesCloseRequestParams<'a> {
        IndicesCloseRequestParams {
            url_params: IndicesCloseUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesCloseRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesCloseUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(8usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_close");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesCloseRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum DeleteScriptUrlParams<'a> {
    LangId(Lang<'a>, Id<'a>),
}
pub struct DeleteScriptRequestParams<'a> {
    pub url_params: DeleteScriptUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> DeleteScriptRequestParams<'a> {
    pub fn lang_id<ILang: Into<Lang<'a>>, IId: Into<Id<'a>>>(lang: ILang,
                                                             id: IId)
                                                             -> DeleteScriptRequestParams<'a> {
        DeleteScriptRequestParams {
            url_params: DeleteScriptUrlParams::LangId(lang.into(), id.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> DeleteScriptRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            DeleteScriptUrlParams::LangId(ref lang, ref id) => {
                let mut url = String::with_capacity(11usize + lang.len() + id.len());
                url.push_str("/_scripts/");
                url.push_str(lang.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a DeleteScriptRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum TermvectorsUrlParams<'a> {
    IndexType(Index<'a>, Type<'a>),
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
pub struct TermvectorsRequestParams<'a> {
    pub url_params: TermvectorsUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> TermvectorsRequestParams<'a> {
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> TermvectorsRequestParams<'a> {
        TermvectorsRequestParams {
            url_params: TermvectorsUrlParams::IndexType(index.into(), ty.into()),
            body: body.into(),
            _a: PhantomData,
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
            url_params: TermvectorsUrlParams::IndexTypeId(index.into(), ty.into(), id.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> TermvectorsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            TermvectorsUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(15usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_termvectors");
                Cow::Owned(url)
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
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a TermvectorsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum FieldStatsUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct FieldStatsRequestParams<'a> {
    pub url_params: FieldStatsUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> FieldStatsRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> FieldStatsRequestParams<'a> {
        FieldStatsRequestParams {
            url_params: FieldStatsUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> FieldStatsRequestParams<'a> {
        FieldStatsRequestParams {
            url_params: FieldStatsUrlParams::Index(index.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> FieldStatsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            FieldStatsUrlParams::None => Cow::Borrowed("/_field_stats"),
            FieldStatsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(14usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_field_stats");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a FieldStatsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum CatThreadPoolUrlParams<'a> {
    None,
    ThreadPoolPatterns(ThreadPoolPatterns<'a>),
}
pub struct CatThreadPoolRequestParams<'a> {
    pub url_params: CatThreadPoolUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> CatThreadPoolRequestParams<'a> {
    pub fn new() -> CatThreadPoolRequestParams<'a> {
        CatThreadPoolRequestParams {
            url_params: CatThreadPoolUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn thread_pool_patterns<IThreadPoolPatterns: Into<ThreadPoolPatterns<'a>>>
        (thread_pool_patterns: IThreadPoolPatterns)
         -> CatThreadPoolRequestParams<'a> {
        CatThreadPoolRequestParams {
            url_params: CatThreadPoolUrlParams::ThreadPoolPatterns(thread_pool_patterns.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> CatThreadPoolRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CatThreadPoolUrlParams::None => Cow::Borrowed("/_cat/thread_pool"),
            CatThreadPoolUrlParams::ThreadPoolPatterns(ref thread_pool_patterns) => {
                let mut url = String::with_capacity(18usize + thread_pool_patterns.len());
                url.push_str("/_cat/thread_pool/");
                url.push_str(thread_pool_patterns.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatThreadPoolRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum SnapshotDeleteUrlParams<'a> {
    RepositorySnapshot(Repository<'a>, Snapshot<'a>),
}
pub struct SnapshotDeleteRequestParams<'a> {
    pub url_params: SnapshotDeleteUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> SnapshotDeleteRequestParams<'a> {
    pub fn repository_snapshot<IRepository: Into<Repository<'a>>, ISnapshot: Into<Snapshot<'a>>>
        (repository: IRepository,
         snapshot: ISnapshot)
         -> SnapshotDeleteRequestParams<'a> {
        SnapshotDeleteRequestParams {
            url_params: SnapshotDeleteUrlParams::RepositorySnapshot(repository.into(),
                                                                    snapshot.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> SnapshotDeleteRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            SnapshotDeleteUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
                let mut url = String::with_capacity(12usize + repository.len() + snapshot.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                url.push_str("/");
                url.push_str(snapshot.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SnapshotDeleteRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesGetSettingsUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexName(Index<'a>, Name<'a>),
    Name(Name<'a>),
}
pub struct IndicesGetSettingsRequestParams<'a> {
    pub url_params: IndicesGetSettingsUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesGetSettingsRequestParams<'a> {
    pub fn new() -> IndicesGetSettingsRequestParams<'a> {
        IndicesGetSettingsRequestParams {
            url_params: IndicesGetSettingsUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesGetSettingsRequestParams<'a> {
        IndicesGetSettingsRequestParams {
            url_params: IndicesGetSettingsUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
    pub fn index_name<IIndex: Into<Index<'a>>, IName: Into<Name<'a>>>
        (index: IIndex,
         name: IName)
         -> IndicesGetSettingsRequestParams<'a> {
        IndicesGetSettingsRequestParams {
            url_params: IndicesGetSettingsUrlParams::IndexName(index.into(), name.into()),
            _a: PhantomData,
        }
    }
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> IndicesGetSettingsRequestParams<'a> {
        IndicesGetSettingsRequestParams {
            url_params: IndicesGetSettingsUrlParams::Name(name.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesGetSettingsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesGetSettingsUrlParams::None => Cow::Borrowed("/_settings"),
            IndicesGetSettingsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(11usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_settings");
                Cow::Owned(url)
            }
            IndicesGetSettingsUrlParams::IndexName(ref index, ref name) => {
                let mut url = String::with_capacity(12usize + index.len() + name.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_settings/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
            IndicesGetSettingsUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(11usize + name.len());
                url.push_str("/_settings/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesGetSettingsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum CreateUrlParams<'a> {
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
pub struct CreateRequestParams<'a> {
    pub url_params: CreateUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
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
            url_params: CreateUrlParams::IndexTypeId(index.into(), ty.into(), id.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> CreateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CreateUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                let mut url = String::with_capacity(11usize + index.len() + ty.len() + id.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                url.push_str("/_create");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CreateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum SnapshotDeleteRepositoryUrlParams<'a> {
    Repository(Repository<'a>),
}
pub struct SnapshotDeleteRepositoryRequestParams<'a> {
    pub url_params: SnapshotDeleteRepositoryUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> SnapshotDeleteRepositoryRequestParams<'a> {
    pub fn repository<IRepository: Into<Repository<'a>>>
        (repository: IRepository)
         -> SnapshotDeleteRepositoryRequestParams<'a> {
        SnapshotDeleteRepositoryRequestParams {
            url_params: SnapshotDeleteRepositoryUrlParams::Repository(repository.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> SnapshotDeleteRepositoryRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            SnapshotDeleteRepositoryUrlParams::Repository(ref repository) => {
                let mut url = String::with_capacity(11usize + repository.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SnapshotDeleteRepositoryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum ClusterAllocationExplainUrlParams {
    None,
}
pub struct ClusterAllocationExplainRequestParams<'a> {
    pub url_params: ClusterAllocationExplainUrlParams,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> ClusterAllocationExplainRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> ClusterAllocationExplainRequestParams<'a> {
        ClusterAllocationExplainRequestParams {
            url_params: ClusterAllocationExplainUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> ClusterAllocationExplainRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ClusterAllocationExplainUrlParams::None => Cow::Borrowed("/_cluster/allocation/explain"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ClusterAllocationExplainRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum IndicesPutTemplateUrlParams<'a> {
    Name(Name<'a>),
}
pub struct IndicesPutTemplateRequestParams<'a> {
    pub url_params: IndicesPutTemplateUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesPutTemplateRequestParams<'a> {
    pub fn name<IName: Into<Name<'a>>, IBody: Into<Body<'a>>>
        (name: IName,
         body: IBody)
         -> IndicesPutTemplateRequestParams<'a> {
        IndicesPutTemplateRequestParams {
            url_params: IndicesPutTemplateUrlParams::Name(name.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesPutTemplateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesPutTemplateUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(11usize + name.len());
                url.push_str("/_template/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesPutTemplateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum IndicesGetTemplateUrlParams<'a> {
    None,
    Name(Name<'a>),
}
pub struct IndicesGetTemplateRequestParams<'a> {
    pub url_params: IndicesGetTemplateUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesGetTemplateRequestParams<'a> {
    pub fn new() -> IndicesGetTemplateRequestParams<'a> {
        IndicesGetTemplateRequestParams {
            url_params: IndicesGetTemplateUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> IndicesGetTemplateRequestParams<'a> {
        IndicesGetTemplateRequestParams {
            url_params: IndicesGetTemplateUrlParams::Name(name.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesGetTemplateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesGetTemplateUrlParams::None => Cow::Borrowed("/_template"),
            IndicesGetTemplateUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(11usize + name.len());
                url.push_str("/_template/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesGetTemplateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum ClusterStateUrlParams<'a> {
    None,
    Metric(Metric<'a>),
    MetricIndex(Metric<'a>, Index<'a>),
}
pub struct ClusterStateRequestParams<'a> {
    pub url_params: ClusterStateUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> ClusterStateRequestParams<'a> {
    pub fn new() -> ClusterStateRequestParams<'a> {
        ClusterStateRequestParams {
            url_params: ClusterStateUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn metric<IMetric: Into<Metric<'a>>>(metric: IMetric) -> ClusterStateRequestParams<'a> {
        ClusterStateRequestParams {
            url_params: ClusterStateUrlParams::Metric(metric.into()),
            _a: PhantomData,
        }
    }
    pub fn metric_index<IMetric: Into<Metric<'a>>, IIndex: Into<Index<'a>>>
        (metric: IMetric,
         index: IIndex)
         -> ClusterStateRequestParams<'a> {
        ClusterStateRequestParams {
            url_params: ClusterStateUrlParams::MetricIndex(metric.into(), index.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> ClusterStateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ClusterStateUrlParams::None => Cow::Borrowed("/_cluster/state"),
            ClusterStateUrlParams::Metric(ref metric) => {
                let mut url = String::with_capacity(16usize + metric.len());
                url.push_str("/_cluster/state/");
                url.push_str(metric.as_ref());
                Cow::Owned(url)
            }
            ClusterStateUrlParams::MetricIndex(ref metric, ref index) => {
                let mut url = String::with_capacity(17usize + metric.len() + index.len());
                url.push_str("/_cluster/state/");
                url.push_str(metric.as_ref());
                url.push_str("/");
                url.push_str(index.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ClusterStateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum MsearchTemplateUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
pub struct MsearchTemplateRequestParams<'a> {
    pub url_params: MsearchTemplateUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> MsearchTemplateRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> MsearchTemplateRequestParams<'a> {
        MsearchTemplateRequestParams {
            url_params: MsearchTemplateUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> MsearchTemplateRequestParams<'a> {
        MsearchTemplateRequestParams {
            url_params: MsearchTemplateUrlParams::Index(index.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> MsearchTemplateRequestParams<'a> {
        MsearchTemplateRequestParams {
            url_params: MsearchTemplateUrlParams::IndexType(index.into(), ty.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> MsearchTemplateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            MsearchTemplateUrlParams::None => Cow::Borrowed("/_msearch/template"),
            MsearchTemplateUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(19usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_msearch/template");
                Cow::Owned(url)
            }
            MsearchTemplateUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(20usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_msearch/template");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a MsearchTemplateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum BulkUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
pub struct BulkRequestParams<'a> {
    pub url_params: BulkUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> BulkRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> BulkRequestParams<'a> {
        BulkRequestParams {
            url_params: BulkUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> BulkRequestParams<'a> {
        BulkRequestParams {
            url_params: BulkUrlParams::Index(index.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> BulkRequestParams<'a> {
        BulkRequestParams {
            url_params: BulkUrlParams::IndexType(index.into(), ty.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> BulkRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            BulkUrlParams::None => Cow::Borrowed("/_bulk"),
            BulkUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(7usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_bulk");
                Cow::Owned(url)
            }
            BulkUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(8usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_bulk");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a BulkRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum ExplainUrlParams<'a> {
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
pub struct ExplainRequestParams<'a> {
    pub url_params: ExplainUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
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
            url_params: ExplainUrlParams::IndexTypeId(index.into(), ty.into(), id.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> ExplainRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ExplainUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                let mut url = String::with_capacity(12usize + index.len() + ty.len() + id.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                url.push_str("/_explain");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ExplainRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum SuggestUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct SuggestRequestParams<'a> {
    pub url_params: SuggestUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> SuggestRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> SuggestRequestParams<'a> {
        SuggestRequestParams {
            url_params: SuggestUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> SuggestRequestParams<'a> {
        SuggestRequestParams {
            url_params: SuggestUrlParams::Index(index.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> SuggestRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            SuggestUrlParams::None => Cow::Borrowed("/_suggest"),
            SuggestUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(10usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_suggest");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SuggestRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum SnapshotGetRepositoryUrlParams<'a> {
    None,
    Repository(Repository<'a>),
}
pub struct SnapshotGetRepositoryRequestParams<'a> {
    pub url_params: SnapshotGetRepositoryUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> SnapshotGetRepositoryRequestParams<'a> {
    pub fn new() -> SnapshotGetRepositoryRequestParams<'a> {
        SnapshotGetRepositoryRequestParams {
            url_params: SnapshotGetRepositoryUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn repository<IRepository: Into<Repository<'a>>>
        (repository: IRepository)
         -> SnapshotGetRepositoryRequestParams<'a> {
        SnapshotGetRepositoryRequestParams {
            url_params: SnapshotGetRepositoryUrlParams::Repository(repository.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> SnapshotGetRepositoryRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            SnapshotGetRepositoryUrlParams::None => Cow::Borrowed("/_snapshot"),
            SnapshotGetRepositoryUrlParams::Repository(ref repository) => {
                let mut url = String::with_capacity(11usize + repository.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SnapshotGetRepositoryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum RenderSearchTemplateUrlParams<'a> {
    None,
    Id(Id<'a>),
}
pub struct RenderSearchTemplateRequestParams<'a> {
    pub url_params: RenderSearchTemplateUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> RenderSearchTemplateRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> RenderSearchTemplateRequestParams<'a> {
        RenderSearchTemplateRequestParams {
            url_params: RenderSearchTemplateUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn id<IId: Into<Id<'a>>, IBody: Into<Body<'a>>>
        (id: IId,
         body: IBody)
         -> RenderSearchTemplateRequestParams<'a> {
        RenderSearchTemplateRequestParams {
            url_params: RenderSearchTemplateUrlParams::Id(id.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> RenderSearchTemplateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            RenderSearchTemplateUrlParams::None => Cow::Borrowed("/_render/template"),
            RenderSearchTemplateUrlParams::Id(ref id) => {
                let mut url = String::with_capacity(18usize + id.len());
                url.push_str("/_render/template/");
                url.push_str(id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a RenderSearchTemplateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum IndicesStatsUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexMetric(Index<'a>, Metric<'a>),
    Metric(Metric<'a>),
}
pub struct IndicesStatsRequestParams<'a> {
    pub url_params: IndicesStatsUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesStatsRequestParams<'a> {
    pub fn new() -> IndicesStatsRequestParams<'a> {
        IndicesStatsRequestParams {
            url_params: IndicesStatsUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesStatsRequestParams<'a> {
        IndicesStatsRequestParams {
            url_params: IndicesStatsUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
    pub fn index_metric<IIndex: Into<Index<'a>>, IMetric: Into<Metric<'a>>>
        (index: IIndex,
         metric: IMetric)
         -> IndicesStatsRequestParams<'a> {
        IndicesStatsRequestParams {
            url_params: IndicesStatsUrlParams::IndexMetric(index.into(), metric.into()),
            _a: PhantomData,
        }
    }
    pub fn metric<IMetric: Into<Metric<'a>>>(metric: IMetric) -> IndicesStatsRequestParams<'a> {
        IndicesStatsRequestParams {
            url_params: IndicesStatsUrlParams::Metric(metric.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesStatsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesStatsUrlParams::None => Cow::Borrowed("/_stats"),
            IndicesStatsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(8usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_stats");
                Cow::Owned(url)
            }
            IndicesStatsUrlParams::IndexMetric(ref index, ref metric) => {
                let mut url = String::with_capacity(9usize + index.len() + metric.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_stats/");
                url.push_str(metric.as_ref());
                Cow::Owned(url)
            }
            IndicesStatsUrlParams::Metric(ref metric) => {
                let mut url = String::with_capacity(8usize + metric.len());
                url.push_str("/_stats/");
                url.push_str(metric.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesStatsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum CatRepositoriesUrlParams {
    None,
}
pub struct CatRepositoriesRequestParams<'a> {
    pub url_params: CatRepositoriesUrlParams,
    _a: PhantomData<&'a ()>,
}
impl<'a> CatRepositoriesRequestParams<'a> {
    pub fn new() -> CatRepositoriesRequestParams<'a> {
        CatRepositoriesRequestParams {
            url_params: CatRepositoriesUrlParams::None,
            _a: PhantomData,
        }
    }
}
impl<'a> CatRepositoriesRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CatRepositoriesUrlParams::None => Cow::Borrowed("/_cat/repositories"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatRepositoriesRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesForcemergeUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct IndicesForcemergeRequestParams<'a> {
    pub url_params: IndicesForcemergeUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesForcemergeRequestParams<'a> {
    pub fn new() -> IndicesForcemergeRequestParams<'a> {
        IndicesForcemergeRequestParams {
            url_params: IndicesForcemergeUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesForcemergeRequestParams<'a> {
        IndicesForcemergeRequestParams {
            url_params: IndicesForcemergeUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesForcemergeRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesForcemergeUrlParams::None => Cow::Borrowed("/_forcemerge"),
            IndicesForcemergeUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(13usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_forcemerge");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesForcemergeRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum PingUrlParams {
    None,
}
pub struct PingRequestParams<'a> {
    pub url_params: PingUrlParams,
    _a: PhantomData<&'a ()>,
}
impl<'a> PingRequestParams<'a> {
    pub fn new() -> PingRequestParams<'a> {
        PingRequestParams {
            url_params: PingUrlParams::None,
            _a: PhantomData,
        }
    }
}
impl<'a> PingRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            PingUrlParams::None => Cow::Borrowed("/"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a PingRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum TasksGetUrlParams<'a> {
    TaskId(TaskId<'a>),
}
pub struct TasksGetRequestParams<'a> {
    pub url_params: TasksGetUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> TasksGetRequestParams<'a> {
    pub fn task_id<ITaskId: Into<TaskId<'a>>>(task_id: ITaskId) -> TasksGetRequestParams<'a> {
        TasksGetRequestParams {
            url_params: TasksGetUrlParams::TaskId(task_id.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> TasksGetRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            TasksGetUrlParams::TaskId(ref task_id) => {
                let mut url = String::with_capacity(8usize + task_id.len());
                url.push_str("/_tasks/");
                url.push_str(task_id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a TasksGetRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesExistsUrlParams<'a> {
    Index(Index<'a>),
}
pub struct IndicesExistsRequestParams<'a> {
    pub url_params: IndicesExistsUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesExistsRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesExistsRequestParams<'a> {
        IndicesExistsRequestParams {
            url_params: IndicesExistsUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesExistsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesExistsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(1usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesExistsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesFlushSyncedUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct IndicesFlushSyncedRequestParams<'a> {
    pub url_params: IndicesFlushSyncedUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesFlushSyncedRequestParams<'a> {
    pub fn new() -> IndicesFlushSyncedRequestParams<'a> {
        IndicesFlushSyncedRequestParams {
            url_params: IndicesFlushSyncedUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesFlushSyncedRequestParams<'a> {
        IndicesFlushSyncedRequestParams {
            url_params: IndicesFlushSyncedUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesFlushSyncedRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesFlushSyncedUrlParams::None => Cow::Borrowed("/_flush/synced"),
            IndicesFlushSyncedUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(15usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_flush/synced");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesFlushSyncedRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum MsearchUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
pub struct MsearchRequestParams<'a> {
    pub url_params: MsearchUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> MsearchRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> MsearchRequestParams<'a> {
        MsearchRequestParams {
            url_params: MsearchUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> MsearchRequestParams<'a> {
        MsearchRequestParams {
            url_params: MsearchUrlParams::Index(index.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> MsearchRequestParams<'a> {
        MsearchRequestParams {
            url_params: MsearchUrlParams::IndexType(index.into(), ty.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> MsearchRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            MsearchUrlParams::None => Cow::Borrowed("/_msearch"),
            MsearchUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(10usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_msearch");
                Cow::Owned(url)
            }
            MsearchUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(11usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_msearch");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a MsearchRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum InfoUrlParams {
    None,
}
pub struct InfoRequestParams<'a> {
    pub url_params: InfoUrlParams,
    _a: PhantomData<&'a ()>,
}
impl<'a> InfoRequestParams<'a> {
    pub fn new() -> InfoRequestParams<'a> {
        InfoRequestParams {
            url_params: InfoUrlParams::None,
            _a: PhantomData,
        }
    }
}
impl<'a> InfoRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            InfoUrlParams::None => Cow::Borrowed("/"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a InfoRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum SearchTemplateUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
pub struct SearchTemplateRequestParams<'a> {
    pub url_params: SearchTemplateUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> SearchTemplateRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> SearchTemplateRequestParams<'a> {
        SearchTemplateRequestParams {
            url_params: SearchTemplateUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> SearchTemplateRequestParams<'a> {
        SearchTemplateRequestParams {
            url_params: SearchTemplateUrlParams::Index(index.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> SearchTemplateRequestParams<'a> {
        SearchTemplateRequestParams {
            url_params: SearchTemplateUrlParams::IndexType(index.into(), ty.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> SearchTemplateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            SearchTemplateUrlParams::None => Cow::Borrowed("/_search/template"),
            SearchTemplateUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(18usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_search/template");
                Cow::Owned(url)
            }
            SearchTemplateUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(19usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_search/template");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SearchTemplateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum IndicesDeleteUrlParams<'a> {
    Index(Index<'a>),
}
pub struct IndicesDeleteRequestParams<'a> {
    pub url_params: IndicesDeleteUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesDeleteRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesDeleteRequestParams<'a> {
        IndicesDeleteRequestParams {
            url_params: IndicesDeleteUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesDeleteRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesDeleteUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(1usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesDeleteRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum DeleteByQueryUrlParams<'a> {
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
pub struct DeleteByQueryRequestParams<'a> {
    pub url_params: DeleteByQueryUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> DeleteByQueryRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> DeleteByQueryRequestParams<'a> {
        DeleteByQueryRequestParams {
            url_params: DeleteByQueryUrlParams::Index(index.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> DeleteByQueryRequestParams<'a> {
        DeleteByQueryRequestParams {
            url_params: DeleteByQueryUrlParams::IndexType(index.into(), ty.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> DeleteByQueryRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            DeleteByQueryUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(18usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_delete_by_query");
                Cow::Owned(url)
            }
            DeleteByQueryUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(19usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_delete_by_query");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a DeleteByQueryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum DeleteTemplateUrlParams<'a> {
    Id(Id<'a>),
}
pub struct DeleteTemplateRequestParams<'a> {
    pub url_params: DeleteTemplateUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> DeleteTemplateRequestParams<'a> {
    pub fn id<IId: Into<Id<'a>>>(id: IId) -> DeleteTemplateRequestParams<'a> {
        DeleteTemplateRequestParams {
            url_params: DeleteTemplateUrlParams::Id(id.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> DeleteTemplateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            DeleteTemplateUrlParams::Id(ref id) => {
                let mut url = String::with_capacity(18usize + id.len());
                url.push_str("/_search/template/");
                url.push_str(id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a DeleteTemplateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesCreateUrlParams<'a> {
    Index(Index<'a>),
}
pub struct IndicesCreateRequestParams<'a> {
    pub url_params: IndicesCreateUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesCreateRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> IndicesCreateRequestParams<'a> {
        IndicesCreateRequestParams {
            url_params: IndicesCreateUrlParams::Index(index.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesCreateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesCreateUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(1usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesCreateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum PercolateUrlParams<'a> {
    IndexType(Index<'a>, Type<'a>),
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
pub struct PercolateRequestParams<'a> {
    pub url_params: PercolateUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> PercolateRequestParams<'a> {
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> PercolateRequestParams<'a> {
        PercolateRequestParams {
            url_params: PercolateUrlParams::IndexType(index.into(), ty.into()),
            body: body.into(),
            _a: PhantomData,
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
            url_params: PercolateUrlParams::IndexTypeId(index.into(), ty.into(), id.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> PercolateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            PercolateUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(13usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_percolate");
                Cow::Owned(url)
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
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a PercolateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum SearchUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
pub struct SearchRequestParams<'a> {
    pub url_params: SearchUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> SearchRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> SearchRequestParams<'a> {
        SearchRequestParams {
            url_params: SearchUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> SearchRequestParams<'a> {
        SearchRequestParams {
            url_params: SearchUrlParams::Index(index.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> SearchRequestParams<'a> {
        SearchRequestParams {
            url_params: SearchUrlParams::IndexType(index.into(), ty.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> SearchRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            SearchUrlParams::None => Cow::Borrowed("/_search"),
            SearchUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(9usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_search");
                Cow::Owned(url)
            }
            SearchUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(10usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_search");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SearchRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum CatNodeattrsUrlParams {
    None,
}
pub struct CatNodeattrsRequestParams<'a> {
    pub url_params: CatNodeattrsUrlParams,
    _a: PhantomData<&'a ()>,
}
impl<'a> CatNodeattrsRequestParams<'a> {
    pub fn new() -> CatNodeattrsRequestParams<'a> {
        CatNodeattrsRequestParams {
            url_params: CatNodeattrsUrlParams::None,
            _a: PhantomData,
        }
    }
}
impl<'a> CatNodeattrsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CatNodeattrsUrlParams::None => Cow::Borrowed("/_cat/nodeattrs"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatNodeattrsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum SnapshotVerifyRepositoryUrlParams<'a> {
    Repository(Repository<'a>),
}
pub struct SnapshotVerifyRepositoryRequestParams<'a> {
    pub url_params: SnapshotVerifyRepositoryUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> SnapshotVerifyRepositoryRequestParams<'a> {
    pub fn repository<IRepository: Into<Repository<'a>>>
        (repository: IRepository)
         -> SnapshotVerifyRepositoryRequestParams<'a> {
        SnapshotVerifyRepositoryRequestParams {
            url_params: SnapshotVerifyRepositoryUrlParams::Repository(repository.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> SnapshotVerifyRepositoryRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            SnapshotVerifyRepositoryUrlParams::Repository(ref repository) => {
                let mut url = String::with_capacity(19usize + repository.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                url.push_str("/_verify");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SnapshotVerifyRepositoryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum CountUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
pub struct CountRequestParams<'a> {
    pub url_params: CountUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> CountRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> CountRequestParams<'a> {
        CountRequestParams {
            url_params: CountUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> CountRequestParams<'a> {
        CountRequestParams {
            url_params: CountUrlParams::Index(index.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> CountRequestParams<'a> {
        CountRequestParams {
            url_params: CountUrlParams::IndexType(index.into(), ty.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> CountRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CountUrlParams::None => Cow::Borrowed("/_count"),
            CountUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(8usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_count");
                Cow::Owned(url)
            }
            CountUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(9usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_count");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CountRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum CatAllocationUrlParams<'a> {
    None,
    NodeId(NodeId<'a>),
}
pub struct CatAllocationRequestParams<'a> {
    pub url_params: CatAllocationUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> CatAllocationRequestParams<'a> {
    pub fn new() -> CatAllocationRequestParams<'a> {
        CatAllocationRequestParams {
            url_params: CatAllocationUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn node_id<INodeId: Into<NodeId<'a>>>(node_id: INodeId) -> CatAllocationRequestParams<'a> {
        CatAllocationRequestParams {
            url_params: CatAllocationUrlParams::NodeId(node_id.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> CatAllocationRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CatAllocationUrlParams::None => Cow::Borrowed("/_cat/allocation"),
            CatAllocationUrlParams::NodeId(ref node_id) => {
                let mut url = String::with_capacity(17usize + node_id.len());
                url.push_str("/_cat/allocation/");
                url.push_str(node_id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatAllocationRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesFlushUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct IndicesFlushRequestParams<'a> {
    pub url_params: IndicesFlushUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesFlushRequestParams<'a> {
    pub fn new() -> IndicesFlushRequestParams<'a> {
        IndicesFlushRequestParams {
            url_params: IndicesFlushUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesFlushRequestParams<'a> {
        IndicesFlushRequestParams {
            url_params: IndicesFlushUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesFlushRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesFlushUrlParams::None => Cow::Borrowed("/_flush"),
            IndicesFlushUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(8usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_flush");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesFlushRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesRefreshUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct IndicesRefreshRequestParams<'a> {
    pub url_params: IndicesRefreshUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesRefreshRequestParams<'a> {
    pub fn new() -> IndicesRefreshRequestParams<'a> {
        IndicesRefreshRequestParams {
            url_params: IndicesRefreshUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesRefreshRequestParams<'a> {
        IndicesRefreshRequestParams {
            url_params: IndicesRefreshUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesRefreshRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesRefreshUrlParams::None => Cow::Borrowed("/_refresh"),
            IndicesRefreshUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(10usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_refresh");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesRefreshRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum CatHelpUrlParams {
    None,
}
pub struct CatHelpRequestParams<'a> {
    pub url_params: CatHelpUrlParams,
    _a: PhantomData<&'a ()>,
}
impl<'a> CatHelpRequestParams<'a> {
    pub fn new() -> CatHelpRequestParams<'a> {
        CatHelpRequestParams {
            url_params: CatHelpUrlParams::None,
            _a: PhantomData,
        }
    }
}
impl<'a> CatHelpRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CatHelpUrlParams::None => Cow::Borrowed("/_cat"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatHelpRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum SearchShardsUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct SearchShardsRequestParams<'a> {
    pub url_params: SearchShardsUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> SearchShardsRequestParams<'a> {
    pub fn new() -> SearchShardsRequestParams<'a> {
        SearchShardsRequestParams {
            url_params: SearchShardsUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> SearchShardsRequestParams<'a> {
        SearchShardsRequestParams {
            url_params: SearchShardsUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> SearchShardsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            SearchShardsUrlParams::None => Cow::Borrowed("/_search_shards"),
            SearchShardsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(16usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_search_shards");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SearchShardsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum ClusterHealthUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct ClusterHealthRequestParams<'a> {
    pub url_params: ClusterHealthUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> ClusterHealthRequestParams<'a> {
    pub fn new() -> ClusterHealthRequestParams<'a> {
        ClusterHealthRequestParams {
            url_params: ClusterHealthUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> ClusterHealthRequestParams<'a> {
        ClusterHealthRequestParams {
            url_params: ClusterHealthUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> ClusterHealthRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ClusterHealthUrlParams::None => Cow::Borrowed("/_cluster/health"),
            ClusterHealthUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(17usize + index.len());
                url.push_str("/_cluster/health/");
                url.push_str(index.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ClusterHealthRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesExistsAliasUrlParams<'a> {
    Index(Index<'a>),
    IndexName(Index<'a>, Name<'a>),
    Name(Name<'a>),
}
pub struct IndicesExistsAliasRequestParams<'a> {
    pub url_params: IndicesExistsAliasUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesExistsAliasRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesExistsAliasRequestParams<'a> {
        IndicesExistsAliasRequestParams {
            url_params: IndicesExistsAliasUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
    pub fn index_name<IIndex: Into<Index<'a>>, IName: Into<Name<'a>>>
        (index: IIndex,
         name: IName)
         -> IndicesExistsAliasRequestParams<'a> {
        IndicesExistsAliasRequestParams {
            url_params: IndicesExistsAliasUrlParams::IndexName(index.into(), name.into()),
            _a: PhantomData,
        }
    }
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> IndicesExistsAliasRequestParams<'a> {
        IndicesExistsAliasRequestParams {
            url_params: IndicesExistsAliasUrlParams::Name(name.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesExistsAliasRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesExistsAliasUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(8usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_alias");
                Cow::Owned(url)
            }
            IndicesExistsAliasUrlParams::IndexName(ref index, ref name) => {
                let mut url = String::with_capacity(9usize + index.len() + name.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_alias/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
            IndicesExistsAliasUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(8usize + name.len());
                url.push_str("/_alias/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesExistsAliasRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesGetFieldMappingUrlParams<'a> {
    Fields(Fields<'a>),
    IndexFields(Index<'a>, Fields<'a>),
    IndexTypeFields(Index<'a>, Type<'a>, Fields<'a>),
    TypeFields(Type<'a>, Fields<'a>),
}
pub struct IndicesGetFieldMappingRequestParams<'a> {
    pub url_params: IndicesGetFieldMappingUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesGetFieldMappingRequestParams<'a> {
    pub fn fields<IFields: Into<Fields<'a>>>(fields: IFields)
                                             -> IndicesGetFieldMappingRequestParams<'a> {
        IndicesGetFieldMappingRequestParams {
            url_params: IndicesGetFieldMappingUrlParams::Fields(fields.into()),
            _a: PhantomData,
        }
    }
    pub fn index_fields<IIndex: Into<Index<'a>>, IFields: Into<Fields<'a>>>
        (index: IIndex,
         fields: IFields)
         -> IndicesGetFieldMappingRequestParams<'a> {
        IndicesGetFieldMappingRequestParams {
            url_params: IndicesGetFieldMappingUrlParams::IndexFields(index.into(), fields.into()),
            _a: PhantomData,
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
            url_params: IndicesGetFieldMappingUrlParams::IndexTypeFields(index.into(),
                                                                         ty.into(),
                                                                         fields.into()),
            _a: PhantomData,
        }
    }
    pub fn ty_fields<IType: Into<Type<'a>>, IFields: Into<Fields<'a>>>
        (ty: IType,
         fields: IFields)
         -> IndicesGetFieldMappingRequestParams<'a> {
        IndicesGetFieldMappingRequestParams {
            url_params: IndicesGetFieldMappingUrlParams::TypeFields(ty.into(), fields.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesGetFieldMappingRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesGetFieldMappingUrlParams::Fields(ref fields) => {
                let mut url = String::with_capacity(16usize + fields.len());
                url.push_str("/_mapping/field/");
                url.push_str(fields.as_ref());
                Cow::Owned(url)
            }
            IndicesGetFieldMappingUrlParams::IndexFields(ref index, ref fields) => {
                let mut url = String::with_capacity(17usize + index.len() + fields.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_mapping/field/");
                url.push_str(fields.as_ref());
                Cow::Owned(url)
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
                Cow::Owned(url)
            }
            IndicesGetFieldMappingUrlParams::TypeFields(ref ty, ref fields) => {
                let mut url = String::with_capacity(17usize + ty.len() + fields.len());
                url.push_str("/_mapping/");
                url.push_str(ty.as_ref());
                url.push_str("/field/");
                url.push_str(fields.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesGetFieldMappingRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IngestPutPipelineUrlParams<'a> {
    Id(Id<'a>),
}
pub struct IngestPutPipelineRequestParams<'a> {
    pub url_params: IngestPutPipelineUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IngestPutPipelineRequestParams<'a> {
    pub fn id<IId: Into<Id<'a>>, IBody: Into<Body<'a>>>(id: IId,
                                                        body: IBody)
                                                        -> IngestPutPipelineRequestParams<'a> {
        IngestPutPipelineRequestParams {
            url_params: IngestPutPipelineUrlParams::Id(id.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> IngestPutPipelineRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IngestPutPipelineUrlParams::Id(ref id) => {
                let mut url = String::with_capacity(18usize + id.len());
                url.push_str("/_ingest/pipeline/");
                url.push_str(id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IngestPutPipelineRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum ClusterPendingTasksUrlParams {
    None,
}
pub struct ClusterPendingTasksRequestParams<'a> {
    pub url_params: ClusterPendingTasksUrlParams,
    _a: PhantomData<&'a ()>,
}
impl<'a> ClusterPendingTasksRequestParams<'a> {
    pub fn new() -> ClusterPendingTasksRequestParams<'a> {
        ClusterPendingTasksRequestParams {
            url_params: ClusterPendingTasksUrlParams::None,
            _a: PhantomData,
        }
    }
}
impl<'a> ClusterPendingTasksRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ClusterPendingTasksUrlParams::None => Cow::Borrowed("/_cluster/pending_tasks"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ClusterPendingTasksRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IngestSimulateUrlParams<'a> {
    None,
    Id(Id<'a>),
}
pub struct IngestSimulateRequestParams<'a> {
    pub url_params: IngestSimulateUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IngestSimulateRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> IngestSimulateRequestParams<'a> {
        IngestSimulateRequestParams {
            url_params: IngestSimulateUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn id<IId: Into<Id<'a>>, IBody: Into<Body<'a>>>(id: IId,
                                                        body: IBody)
                                                        -> IngestSimulateRequestParams<'a> {
        IngestSimulateRequestParams {
            url_params: IngestSimulateUrlParams::Id(id.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> IngestSimulateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IngestSimulateUrlParams::None => Cow::Borrowed("/_ingest/pipeline/_simulate"),
            IngestSimulateUrlParams::Id(ref id) => {
                let mut url = String::with_capacity(28usize + id.len());
                url.push_str("/_ingest/pipeline/");
                url.push_str(id.as_ref());
                url.push_str("/_simulate");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IngestSimulateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum IndicesGetAliasUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexName(Index<'a>, Name<'a>),
    Name(Name<'a>),
}
pub struct IndicesGetAliasRequestParams<'a> {
    pub url_params: IndicesGetAliasUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesGetAliasRequestParams<'a> {
    pub fn new() -> IndicesGetAliasRequestParams<'a> {
        IndicesGetAliasRequestParams {
            url_params: IndicesGetAliasUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesGetAliasRequestParams<'a> {
        IndicesGetAliasRequestParams {
            url_params: IndicesGetAliasUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
    pub fn index_name<IIndex: Into<Index<'a>>, IName: Into<Name<'a>>>
        (index: IIndex,
         name: IName)
         -> IndicesGetAliasRequestParams<'a> {
        IndicesGetAliasRequestParams {
            url_params: IndicesGetAliasUrlParams::IndexName(index.into(), name.into()),
            _a: PhantomData,
        }
    }
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> IndicesGetAliasRequestParams<'a> {
        IndicesGetAliasRequestParams {
            url_params: IndicesGetAliasUrlParams::Name(name.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesGetAliasRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesGetAliasUrlParams::None => Cow::Borrowed("/_alias"),
            IndicesGetAliasUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(8usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_alias");
                Cow::Owned(url)
            }
            IndicesGetAliasUrlParams::IndexName(ref index, ref name) => {
                let mut url = String::with_capacity(9usize + index.len() + name.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_alias/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
            IndicesGetAliasUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(8usize + name.len());
                url.push_str("/_alias/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesGetAliasRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum GetScriptUrlParams<'a> {
    LangId(Lang<'a>, Id<'a>),
}
pub struct GetScriptRequestParams<'a> {
    pub url_params: GetScriptUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> GetScriptRequestParams<'a> {
    pub fn lang_id<ILang: Into<Lang<'a>>, IId: Into<Id<'a>>>(lang: ILang,
                                                             id: IId)
                                                             -> GetScriptRequestParams<'a> {
        GetScriptRequestParams {
            url_params: GetScriptUrlParams::LangId(lang.into(), id.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> GetScriptRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            GetScriptUrlParams::LangId(ref lang, ref id) => {
                let mut url = String::with_capacity(11usize + lang.len() + id.len());
                url.push_str("/_scripts/");
                url.push_str(lang.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a GetScriptRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesRecoveryUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct IndicesRecoveryRequestParams<'a> {
    pub url_params: IndicesRecoveryUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesRecoveryRequestParams<'a> {
    pub fn new() -> IndicesRecoveryRequestParams<'a> {
        IndicesRecoveryRequestParams {
            url_params: IndicesRecoveryUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesRecoveryRequestParams<'a> {
        IndicesRecoveryRequestParams {
            url_params: IndicesRecoveryUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesRecoveryRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesRecoveryUrlParams::None => Cow::Borrowed("/_recovery"),
            IndicesRecoveryUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(11usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_recovery");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesRecoveryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IngestDeletePipelineUrlParams<'a> {
    Id(Id<'a>),
}
pub struct IngestDeletePipelineRequestParams<'a> {
    pub url_params: IngestDeletePipelineUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IngestDeletePipelineRequestParams<'a> {
    pub fn id<IId: Into<Id<'a>>>(id: IId) -> IngestDeletePipelineRequestParams<'a> {
        IngestDeletePipelineRequestParams {
            url_params: IngestDeletePipelineUrlParams::Id(id.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IngestDeletePipelineRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IngestDeletePipelineUrlParams::Id(ref id) => {
                let mut url = String::with_capacity(18usize + id.len());
                url.push_str("/_ingest/pipeline/");
                url.push_str(id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IngestDeletePipelineRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum TasksCancelUrlParams<'a> {
    None,
    TaskId(TaskId<'a>),
}
pub struct TasksCancelRequestParams<'a> {
    pub url_params: TasksCancelUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> TasksCancelRequestParams<'a> {
    pub fn new() -> TasksCancelRequestParams<'a> {
        TasksCancelRequestParams {
            url_params: TasksCancelUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn task_id<ITaskId: Into<TaskId<'a>>>(task_id: ITaskId) -> TasksCancelRequestParams<'a> {
        TasksCancelRequestParams {
            url_params: TasksCancelUrlParams::TaskId(task_id.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> TasksCancelRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            TasksCancelUrlParams::None => Cow::Borrowed("/_tasks/_cancel"),
            TasksCancelUrlParams::TaskId(ref task_id) => {
                let mut url = String::with_capacity(16usize + task_id.len());
                url.push_str("/_tasks/");
                url.push_str(task_id.as_ref());
                url.push_str("/_cancel");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a TasksCancelRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesClearCacheUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct IndicesClearCacheRequestParams<'a> {
    pub url_params: IndicesClearCacheUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesClearCacheRequestParams<'a> {
    pub fn new() -> IndicesClearCacheRequestParams<'a> {
        IndicesClearCacheRequestParams {
            url_params: IndicesClearCacheUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesClearCacheRequestParams<'a> {
        IndicesClearCacheRequestParams {
            url_params: IndicesClearCacheUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesClearCacheRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesClearCacheUrlParams::None => Cow::Borrowed("/_cache/clear"),
            IndicesClearCacheUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(14usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_cache/clear");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesClearCacheRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum DeleteUrlParams<'a> {
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
pub struct DeleteRequestParams<'a> {
    pub url_params: DeleteUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> DeleteRequestParams<'a> {
    pub fn index_ty_id<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IId: Into<Id<'a>>>
        (index: IIndex,
         ty: IType,
         id: IId)
         -> DeleteRequestParams<'a> {
        DeleteRequestParams {
            url_params: DeleteUrlParams::IndexTypeId(index.into(), ty.into(), id.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> DeleteRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            DeleteUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                let mut url = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a DeleteRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesPutMappingUrlParams<'a> {
    IndexType(Index<'a>, Type<'a>),
    Type(Type<'a>),
}
pub struct IndicesPutMappingRequestParams<'a> {
    pub url_params: IndicesPutMappingUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesPutMappingRequestParams<'a> {
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> IndicesPutMappingRequestParams<'a> {
        IndicesPutMappingRequestParams {
            url_params: IndicesPutMappingUrlParams::IndexType(index.into(), ty.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn ty<IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (ty: IType,
         body: IBody)
         -> IndicesPutMappingRequestParams<'a> {
        IndicesPutMappingRequestParams {
            url_params: IndicesPutMappingUrlParams::Type(ty.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesPutMappingRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesPutMappingUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(12usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_mappings/");
                url.push_str(ty.as_ref());
                Cow::Owned(url)
            }
            IndicesPutMappingUrlParams::Type(ref ty) => {
                let mut url = String::with_capacity(11usize + ty.len());
                url.push_str("/_mappings/");
                url.push_str(ty.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesPutMappingRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum CatAliasesUrlParams<'a> {
    None,
    Name(Name<'a>),
}
pub struct CatAliasesRequestParams<'a> {
    pub url_params: CatAliasesUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> CatAliasesRequestParams<'a> {
    pub fn new() -> CatAliasesRequestParams<'a> {
        CatAliasesRequestParams {
            url_params: CatAliasesUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> CatAliasesRequestParams<'a> {
        CatAliasesRequestParams {
            url_params: CatAliasesUrlParams::Name(name.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> CatAliasesRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CatAliasesUrlParams::None => Cow::Borrowed("/_cat/aliases"),
            CatAliasesUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(14usize + name.len());
                url.push_str("/_cat/aliases/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatAliasesRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum ClusterStatsUrlParams<'a> {
    None,
    NodeId(NodeId<'a>),
}
pub struct ClusterStatsRequestParams<'a> {
    pub url_params: ClusterStatsUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> ClusterStatsRequestParams<'a> {
    pub fn new() -> ClusterStatsRequestParams<'a> {
        ClusterStatsRequestParams {
            url_params: ClusterStatsUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn node_id<INodeId: Into<NodeId<'a>>>(node_id: INodeId) -> ClusterStatsRequestParams<'a> {
        ClusterStatsRequestParams {
            url_params: ClusterStatsUrlParams::NodeId(node_id.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> ClusterStatsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ClusterStatsUrlParams::None => Cow::Borrowed("/_cluster/stats"),
            ClusterStatsUrlParams::NodeId(ref node_id) => {
                let mut url = String::with_capacity(22usize + node_id.len());
                url.push_str("/_cluster/stats/nodes/");
                url.push_str(node_id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ClusterStatsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesValidateQueryUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
pub struct IndicesValidateQueryRequestParams<'a> {
    pub url_params: IndicesValidateQueryUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesValidateQueryRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> IndicesValidateQueryRequestParams<'a> {
        IndicesValidateQueryRequestParams {
            url_params: IndicesValidateQueryUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> IndicesValidateQueryRequestParams<'a> {
        IndicesValidateQueryRequestParams {
            url_params: IndicesValidateQueryUrlParams::Index(index.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> IndicesValidateQueryRequestParams<'a> {
        IndicesValidateQueryRequestParams {
            url_params: IndicesValidateQueryUrlParams::IndexType(index.into(), ty.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesValidateQueryRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesValidateQueryUrlParams::None => Cow::Borrowed("/_validate/query"),
            IndicesValidateQueryUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(17usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_validate/query");
                Cow::Owned(url)
            }
            IndicesValidateQueryUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(18usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_validate/query");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesValidateQueryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum CatPendingTasksUrlParams {
    None,
}
pub struct CatPendingTasksRequestParams<'a> {
    pub url_params: CatPendingTasksUrlParams,
    _a: PhantomData<&'a ()>,
}
impl<'a> CatPendingTasksRequestParams<'a> {
    pub fn new() -> CatPendingTasksRequestParams<'a> {
        CatPendingTasksRequestParams {
            url_params: CatPendingTasksUrlParams::None,
            _a: PhantomData,
        }
    }
}
impl<'a> CatPendingTasksRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CatPendingTasksUrlParams::None => Cow::Borrowed("/_cat/pending_tasks"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatPendingTasksRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum ClearScrollUrlParams<'a> {
    None,
    ScrollId(ScrollId<'a>),
}
pub struct ClearScrollRequestParams<'a> {
    pub url_params: ClearScrollUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> ClearScrollRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> ClearScrollRequestParams<'a> {
        ClearScrollRequestParams {
            url_params: ClearScrollUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn scroll_id<IScrollId: Into<ScrollId<'a>>, IBody: Into<Body<'a>>>
        (scroll_id: IScrollId,
         body: IBody)
         -> ClearScrollRequestParams<'a> {
        ClearScrollRequestParams {
            url_params: ClearScrollUrlParams::ScrollId(scroll_id.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> ClearScrollRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ClearScrollUrlParams::None => Cow::Borrowed("/_search/scroll"),
            ClearScrollUrlParams::ScrollId(ref scroll_id) => {
                let mut url = String::with_capacity(16usize + scroll_id.len());
                url.push_str("/_search/scroll/");
                url.push_str(scroll_id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ClearScrollRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum CatShardsUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct CatShardsRequestParams<'a> {
    pub url_params: CatShardsUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> CatShardsRequestParams<'a> {
    pub fn new() -> CatShardsRequestParams<'a> {
        CatShardsRequestParams {
            url_params: CatShardsUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> CatShardsRequestParams<'a> {
        CatShardsRequestParams {
            url_params: CatShardsUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> CatShardsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CatShardsUrlParams::None => Cow::Borrowed("/_cat/shards"),
            CatShardsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(13usize + index.len());
                url.push_str("/_cat/shards/");
                url.push_str(index.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatShardsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesShardStoresUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct IndicesShardStoresRequestParams<'a> {
    pub url_params: IndicesShardStoresUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesShardStoresRequestParams<'a> {
    pub fn new() -> IndicesShardStoresRequestParams<'a> {
        IndicesShardStoresRequestParams {
            url_params: IndicesShardStoresUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesShardStoresRequestParams<'a> {
        IndicesShardStoresRequestParams {
            url_params: IndicesShardStoresUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesShardStoresRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesShardStoresUrlParams::None => Cow::Borrowed("/_shard_stores"),
            IndicesShardStoresUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(15usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_shard_stores");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesShardStoresRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesUpdateAliasesUrlParams {
    None,
}
pub struct IndicesUpdateAliasesRequestParams<'a> {
    pub url_params: IndicesUpdateAliasesUrlParams,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesUpdateAliasesRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> IndicesUpdateAliasesRequestParams<'a> {
        IndicesUpdateAliasesRequestParams {
            url_params: IndicesUpdateAliasesUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesUpdateAliasesRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesUpdateAliasesUrlParams::None => Cow::Borrowed("/_aliases"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesUpdateAliasesRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum CatSegmentsUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct CatSegmentsRequestParams<'a> {
    pub url_params: CatSegmentsUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> CatSegmentsRequestParams<'a> {
    pub fn new() -> CatSegmentsRequestParams<'a> {
        CatSegmentsRequestParams {
            url_params: CatSegmentsUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> CatSegmentsRequestParams<'a> {
        CatSegmentsRequestParams {
            url_params: CatSegmentsUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> CatSegmentsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CatSegmentsUrlParams::None => Cow::Borrowed("/_cat/segments"),
            CatSegmentsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(15usize + index.len());
                url.push_str("/_cat/segments/");
                url.push_str(index.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatSegmentsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum MpercolateUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
pub struct MpercolateRequestParams<'a> {
    pub url_params: MpercolateUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> MpercolateRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> MpercolateRequestParams<'a> {
        MpercolateRequestParams {
            url_params: MpercolateUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> MpercolateRequestParams<'a> {
        MpercolateRequestParams {
            url_params: MpercolateUrlParams::Index(index.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> MpercolateRequestParams<'a> {
        MpercolateRequestParams {
            url_params: MpercolateUrlParams::IndexType(index.into(), ty.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> MpercolateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            MpercolateUrlParams::None => Cow::Borrowed("/_mpercolate"),
            MpercolateUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(13usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_mpercolate");
                Cow::Owned(url)
            }
            MpercolateUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(14usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_mpercolate");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a MpercolateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum IndicesOpenUrlParams<'a> {
    Index(Index<'a>),
}
pub struct IndicesOpenRequestParams<'a> {
    pub url_params: IndicesOpenUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesOpenRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesOpenRequestParams<'a> {
        IndicesOpenRequestParams {
            url_params: IndicesOpenUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesOpenRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesOpenUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(7usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_open");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesOpenRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum GetUrlParams<'a> {
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
pub struct GetRequestParams<'a> {
    pub url_params: GetUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> GetRequestParams<'a> {
    pub fn index_ty_id<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IId: Into<Id<'a>>>
        (index: IIndex,
         ty: IType,
         id: IId)
         -> GetRequestParams<'a> {
        GetRequestParams {
            url_params: GetUrlParams::IndexTypeId(index.into(), ty.into(), id.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> GetRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            GetUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                let mut url = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a GetRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum UpdateByQueryUrlParams<'a> {
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
pub struct UpdateByQueryRequestParams<'a> {
    pub url_params: UpdateByQueryUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> UpdateByQueryRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> UpdateByQueryRequestParams<'a> {
        UpdateByQueryRequestParams {
            url_params: UpdateByQueryUrlParams::Index(index.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> UpdateByQueryRequestParams<'a> {
        UpdateByQueryRequestParams {
            url_params: UpdateByQueryUrlParams::IndexType(index.into(), ty.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> UpdateByQueryRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            UpdateByQueryUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(18usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_update_by_query");
                Cow::Owned(url)
            }
            UpdateByQueryUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(19usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_update_by_query");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a UpdateByQueryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum MtermvectorsUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
pub struct MtermvectorsRequestParams<'a> {
    pub url_params: MtermvectorsUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> MtermvectorsRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> MtermvectorsRequestParams<'a> {
        MtermvectorsRequestParams {
            url_params: MtermvectorsUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> MtermvectorsRequestParams<'a> {
        MtermvectorsRequestParams {
            url_params: MtermvectorsUrlParams::Index(index.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> MtermvectorsRequestParams<'a> {
        MtermvectorsRequestParams {
            url_params: MtermvectorsUrlParams::IndexType(index.into(), ty.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> MtermvectorsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            MtermvectorsUrlParams::None => Cow::Borrowed("/_mtermvectors"),
            MtermvectorsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(15usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_mtermvectors");
                Cow::Owned(url)
            }
            MtermvectorsUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(16usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_mtermvectors");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a MtermvectorsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum CatRecoveryUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct CatRecoveryRequestParams<'a> {
    pub url_params: CatRecoveryUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> CatRecoveryRequestParams<'a> {
    pub fn new() -> CatRecoveryRequestParams<'a> {
        CatRecoveryRequestParams {
            url_params: CatRecoveryUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> CatRecoveryRequestParams<'a> {
        CatRecoveryRequestParams {
            url_params: CatRecoveryUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> CatRecoveryRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CatRecoveryUrlParams::None => Cow::Borrowed("/_cat/recovery"),
            CatRecoveryUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(15usize + index.len());
                url.push_str("/_cat/recovery/");
                url.push_str(index.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatRecoveryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum SnapshotRestoreUrlParams<'a> {
    RepositorySnapshot(Repository<'a>, Snapshot<'a>),
}
pub struct SnapshotRestoreRequestParams<'a> {
    pub url_params: SnapshotRestoreUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
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
            url_params: SnapshotRestoreUrlParams::RepositorySnapshot(repository.into(),
                                                                     snapshot.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> SnapshotRestoreRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            SnapshotRestoreUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
                let mut url = String::with_capacity(21usize + repository.len() + snapshot.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                url.push_str("/");
                url.push_str(snapshot.as_ref());
                url.push_str("/_restore");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SnapshotRestoreRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum ReindexUrlParams {
    None,
}
pub struct ReindexRequestParams<'a> {
    pub url_params: ReindexUrlParams,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> ReindexRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> ReindexRequestParams<'a> {
        ReindexRequestParams {
            url_params: ReindexUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> ReindexRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ReindexUrlParams::None => Cow::Borrowed("/_reindex"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ReindexRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum CatHealthUrlParams {
    None,
}
pub struct CatHealthRequestParams<'a> {
    pub url_params: CatHealthUrlParams,
    _a: PhantomData<&'a ()>,
}
impl<'a> CatHealthRequestParams<'a> {
    pub fn new() -> CatHealthRequestParams<'a> {
        CatHealthRequestParams {
            url_params: CatHealthUrlParams::None,
            _a: PhantomData,
        }
    }
}
impl<'a> CatHealthRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CatHealthUrlParams::None => Cow::Borrowed("/_cat/health"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatHealthRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum CatCountUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct CatCountRequestParams<'a> {
    pub url_params: CatCountUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> CatCountRequestParams<'a> {
    pub fn new() -> CatCountRequestParams<'a> {
        CatCountRequestParams {
            url_params: CatCountUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> CatCountRequestParams<'a> {
        CatCountRequestParams {
            url_params: CatCountUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> CatCountRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CatCountUrlParams::None => Cow::Borrowed("/_cat/count"),
            CatCountUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(12usize + index.len());
                url.push_str("/_cat/count/");
                url.push_str(index.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatCountRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum CatSnapshotsUrlParams<'a> {
    None,
    Repository(Repository<'a>),
}
pub struct CatSnapshotsRequestParams<'a> {
    pub url_params: CatSnapshotsUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> CatSnapshotsRequestParams<'a> {
    pub fn new() -> CatSnapshotsRequestParams<'a> {
        CatSnapshotsRequestParams {
            url_params: CatSnapshotsUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn repository<IRepository: Into<Repository<'a>>>(repository: IRepository)
                                                         -> CatSnapshotsRequestParams<'a> {
        CatSnapshotsRequestParams {
            url_params: CatSnapshotsUrlParams::Repository(repository.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> CatSnapshotsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CatSnapshotsUrlParams::None => Cow::Borrowed("/_cat/snapshots"),
            CatSnapshotsUrlParams::Repository(ref repository) => {
                let mut url = String::with_capacity(16usize + repository.len());
                url.push_str("/_cat/snapshots/");
                url.push_str(repository.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatSnapshotsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesGetMappingUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
    Type(Type<'a>),
}
pub struct IndicesGetMappingRequestParams<'a> {
    pub url_params: IndicesGetMappingUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesGetMappingRequestParams<'a> {
    pub fn new() -> IndicesGetMappingRequestParams<'a> {
        IndicesGetMappingRequestParams {
            url_params: IndicesGetMappingUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesGetMappingRequestParams<'a> {
        IndicesGetMappingRequestParams {
            url_params: IndicesGetMappingUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>>
        (index: IIndex,
         ty: IType)
         -> IndicesGetMappingRequestParams<'a> {
        IndicesGetMappingRequestParams {
            url_params: IndicesGetMappingUrlParams::IndexType(index.into(), ty.into()),
            _a: PhantomData,
        }
    }
    pub fn ty<IType: Into<Type<'a>>>(ty: IType) -> IndicesGetMappingRequestParams<'a> {
        IndicesGetMappingRequestParams {
            url_params: IndicesGetMappingUrlParams::Type(ty.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesGetMappingRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesGetMappingUrlParams::None => Cow::Borrowed("/_mapping"),
            IndicesGetMappingUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(10usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_mapping");
                Cow::Owned(url)
            }
            IndicesGetMappingUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(11usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_mapping/");
                url.push_str(ty.as_ref());
                Cow::Owned(url)
            }
            IndicesGetMappingUrlParams::Type(ref ty) => {
                let mut url = String::with_capacity(10usize + ty.len());
                url.push_str("/_mapping/");
                url.push_str(ty.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesGetMappingRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum SnapshotGetUrlParams<'a> {
    RepositorySnapshot(Repository<'a>, Snapshot<'a>),
}
pub struct SnapshotGetRequestParams<'a> {
    pub url_params: SnapshotGetUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> SnapshotGetRequestParams<'a> {
    pub fn repository_snapshot<IRepository: Into<Repository<'a>>, ISnapshot: Into<Snapshot<'a>>>
        (repository: IRepository,
         snapshot: ISnapshot)
         -> SnapshotGetRequestParams<'a> {
        SnapshotGetRequestParams {
            url_params: SnapshotGetUrlParams::RepositorySnapshot(repository.into(),
                                                                 snapshot.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> SnapshotGetRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            SnapshotGetUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
                let mut url = String::with_capacity(12usize + repository.len() + snapshot.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                url.push_str("/");
                url.push_str(snapshot.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SnapshotGetRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum CatNodesUrlParams {
    None,
}
pub struct CatNodesRequestParams<'a> {
    pub url_params: CatNodesUrlParams,
    _a: PhantomData<&'a ()>,
}
impl<'a> CatNodesRequestParams<'a> {
    pub fn new() -> CatNodesRequestParams<'a> {
        CatNodesRequestParams {
            url_params: CatNodesUrlParams::None,
            _a: PhantomData,
        }
    }
}
impl<'a> CatNodesRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CatNodesUrlParams::None => Cow::Borrowed("/_cat/nodes"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatNodesRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum ExistsUrlParams<'a> {
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
pub struct ExistsRequestParams<'a> {
    pub url_params: ExistsUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> ExistsRequestParams<'a> {
    pub fn index_ty_id<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IId: Into<Id<'a>>>
        (index: IIndex,
         ty: IType,
         id: IId)
         -> ExistsRequestParams<'a> {
        ExistsRequestParams {
            url_params: ExistsUrlParams::IndexTypeId(index.into(), ty.into(), id.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> ExistsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ExistsUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                let mut url = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ExistsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum ClusterRerouteUrlParams {
    None,
}
pub struct ClusterRerouteRequestParams<'a> {
    pub url_params: ClusterRerouteUrlParams,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> ClusterRerouteRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> ClusterRerouteRequestParams<'a> {
        ClusterRerouteRequestParams {
            url_params: ClusterRerouteUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> ClusterRerouteRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ClusterRerouteUrlParams::None => Cow::Borrowed("/_cluster/reroute"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ClusterRerouteRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum NodesHotThreadsUrlParams<'a> {
    None,
    NodeId(NodeId<'a>),
}
pub struct NodesHotThreadsRequestParams<'a> {
    pub url_params: NodesHotThreadsUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> NodesHotThreadsRequestParams<'a> {
    pub fn new() -> NodesHotThreadsRequestParams<'a> {
        NodesHotThreadsRequestParams {
            url_params: NodesHotThreadsUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn node_id<INodeId: Into<NodeId<'a>>>(node_id: INodeId)
                                              -> NodesHotThreadsRequestParams<'a> {
        NodesHotThreadsRequestParams {
            url_params: NodesHotThreadsUrlParams::NodeId(node_id.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> NodesHotThreadsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            NodesHotThreadsUrlParams::None => Cow::Borrowed("/_nodes/hot_threads"),
            NodesHotThreadsUrlParams::NodeId(ref node_id) => {
                let mut url = String::with_capacity(20usize + node_id.len());
                url.push_str("/_nodes/");
                url.push_str(node_id.as_ref());
                url.push_str("/hot_threads");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a NodesHotThreadsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum NodesStatsUrlParams<'a> {
    None,
    Metric(Metric<'a>),
    MetricIndexMetric(Metric<'a>, IndexMetric<'a>),
    NodeId(NodeId<'a>),
    NodeIdMetric(NodeId<'a>, Metric<'a>),
    NodeIdMetricIndexMetric(NodeId<'a>, Metric<'a>, IndexMetric<'a>),
}
pub struct NodesStatsRequestParams<'a> {
    pub url_params: NodesStatsUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> NodesStatsRequestParams<'a> {
    pub fn new() -> NodesStatsRequestParams<'a> {
        NodesStatsRequestParams {
            url_params: NodesStatsUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn metric<IMetric: Into<Metric<'a>>>(metric: IMetric) -> NodesStatsRequestParams<'a> {
        NodesStatsRequestParams {
            url_params: NodesStatsUrlParams::Metric(metric.into()),
            _a: PhantomData,
        }
    }
    pub fn metric_index_metric<IMetric: Into<Metric<'a>>, IIndexMetric: Into<IndexMetric<'a>>>
        (metric: IMetric,
         index_metric: IIndexMetric)
         -> NodesStatsRequestParams<'a> {
        NodesStatsRequestParams {
            url_params: NodesStatsUrlParams::MetricIndexMetric(metric.into(), index_metric.into()),
            _a: PhantomData,
        }
    }
    pub fn node_id<INodeId: Into<NodeId<'a>>>(node_id: INodeId) -> NodesStatsRequestParams<'a> {
        NodesStatsRequestParams {
            url_params: NodesStatsUrlParams::NodeId(node_id.into()),
            _a: PhantomData,
        }
    }
    pub fn node_id_metric<INodeId: Into<NodeId<'a>>, IMetric: Into<Metric<'a>>>
        (node_id: INodeId,
         metric: IMetric)
         -> NodesStatsRequestParams<'a> {
        NodesStatsRequestParams {
            url_params: NodesStatsUrlParams::NodeIdMetric(node_id.into(), metric.into()),
            _a: PhantomData,
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
            url_params: NodesStatsUrlParams::NodeIdMetricIndexMetric(node_id.into(),
                                                                     metric.into(),
                                                                     index_metric.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> NodesStatsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            NodesStatsUrlParams::None => Cow::Borrowed("/_nodes/stats"),
            NodesStatsUrlParams::Metric(ref metric) => {
                let mut url = String::with_capacity(14usize + metric.len());
                url.push_str("/_nodes/stats/");
                url.push_str(metric.as_ref());
                Cow::Owned(url)
            }
            NodesStatsUrlParams::MetricIndexMetric(ref metric, ref index_metric) => {
                let mut url = String::with_capacity(15usize + metric.len() + index_metric.len());
                url.push_str("/_nodes/stats/");
                url.push_str(metric.as_ref());
                url.push_str("/");
                url.push_str(index_metric.as_ref());
                Cow::Owned(url)
            }
            NodesStatsUrlParams::NodeId(ref node_id) => {
                let mut url = String::with_capacity(14usize + node_id.len());
                url.push_str("/_nodes/");
                url.push_str(node_id.as_ref());
                url.push_str("/stats");
                Cow::Owned(url)
            }
            NodesStatsUrlParams::NodeIdMetric(ref node_id, ref metric) => {
                let mut url = String::with_capacity(15usize + node_id.len() + metric.len());
                url.push_str("/_nodes/");
                url.push_str(node_id.as_ref());
                url.push_str("/stats/");
                url.push_str(metric.as_ref());
                Cow::Owned(url)
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
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a NodesStatsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IngestGetPipelineUrlParams<'a> {
    None,
    Id(Id<'a>),
}
pub struct IngestGetPipelineRequestParams<'a> {
    pub url_params: IngestGetPipelineUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IngestGetPipelineRequestParams<'a> {
    pub fn new() -> IngestGetPipelineRequestParams<'a> {
        IngestGetPipelineRequestParams {
            url_params: IngestGetPipelineUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn id<IId: Into<Id<'a>>>(id: IId) -> IngestGetPipelineRequestParams<'a> {
        IngestGetPipelineRequestParams {
            url_params: IngestGetPipelineUrlParams::Id(id.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IngestGetPipelineRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IngestGetPipelineUrlParams::None => Cow::Borrowed("/_ingest/pipeline"),
            IngestGetPipelineUrlParams::Id(ref id) => {
                let mut url = String::with_capacity(18usize + id.len());
                url.push_str("/_ingest/pipeline/");
                url.push_str(id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IngestGetPipelineRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum PutTemplateUrlParams<'a> {
    Id(Id<'a>),
}
pub struct PutTemplateRequestParams<'a> {
    pub url_params: PutTemplateUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> PutTemplateRequestParams<'a> {
    pub fn id<IId: Into<Id<'a>>, IBody: Into<Body<'a>>>(id: IId,
                                                        body: IBody)
                                                        -> PutTemplateRequestParams<'a> {
        PutTemplateRequestParams {
            url_params: PutTemplateUrlParams::Id(id.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> PutTemplateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            PutTemplateUrlParams::Id(ref id) => {
                let mut url = String::with_capacity(18usize + id.len());
                url.push_str("/_search/template/");
                url.push_str(id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a PutTemplateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum GetSourceUrlParams<'a> {
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
pub struct GetSourceRequestParams<'a> {
    pub url_params: GetSourceUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> GetSourceRequestParams<'a> {
    pub fn index_ty_id<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IId: Into<Id<'a>>>
        (index: IIndex,
         ty: IType,
         id: IId)
         -> GetSourceRequestParams<'a> {
        GetSourceRequestParams {
            url_params: GetSourceUrlParams::IndexTypeId(index.into(), ty.into(), id.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> GetSourceRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            GetSourceUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                let mut url = String::with_capacity(11usize + index.len() + ty.len() + id.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                url.push_str("/_source");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a GetSourceRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum SnapshotCreateUrlParams<'a> {
    RepositorySnapshot(Repository<'a>, Snapshot<'a>),
}
pub struct SnapshotCreateRequestParams<'a> {
    pub url_params: SnapshotCreateUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
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
            url_params: SnapshotCreateUrlParams::RepositorySnapshot(repository.into(),
                                                                    snapshot.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> SnapshotCreateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            SnapshotCreateUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
                let mut url = String::with_capacity(12usize + repository.len() + snapshot.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                url.push_str("/");
                url.push_str(snapshot.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SnapshotCreateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum ScrollUrlParams<'a> {
    None,
    ScrollId(ScrollId<'a>),
}
pub struct ScrollRequestParams<'a> {
    pub url_params: ScrollUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> ScrollRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> ScrollRequestParams<'a> {
        ScrollRequestParams {
            url_params: ScrollUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn scroll_id<IScrollId: Into<ScrollId<'a>>, IBody: Into<Body<'a>>>
        (scroll_id: IScrollId,
         body: IBody)
         -> ScrollRequestParams<'a> {
        ScrollRequestParams {
            url_params: ScrollUrlParams::ScrollId(scroll_id.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> ScrollRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ScrollUrlParams::None => Cow::Borrowed("/_search/scroll"),
            ScrollUrlParams::ScrollId(ref scroll_id) => {
                let mut url = String::with_capacity(16usize + scroll_id.len());
                url.push_str("/_search/scroll/");
                url.push_str(scroll_id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ScrollRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum SnapshotStatusUrlParams<'a> {
    None,
    Repository(Repository<'a>),
    RepositorySnapshot(Repository<'a>, Snapshot<'a>),
}
pub struct SnapshotStatusRequestParams<'a> {
    pub url_params: SnapshotStatusUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> SnapshotStatusRequestParams<'a> {
    pub fn new() -> SnapshotStatusRequestParams<'a> {
        SnapshotStatusRequestParams {
            url_params: SnapshotStatusUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn repository<IRepository: Into<Repository<'a>>>(repository: IRepository)
                                                         -> SnapshotStatusRequestParams<'a> {
        SnapshotStatusRequestParams {
            url_params: SnapshotStatusUrlParams::Repository(repository.into()),
            _a: PhantomData,
        }
    }
    pub fn repository_snapshot<IRepository: Into<Repository<'a>>, ISnapshot: Into<Snapshot<'a>>>
        (repository: IRepository,
         snapshot: ISnapshot)
         -> SnapshotStatusRequestParams<'a> {
        SnapshotStatusRequestParams {
            url_params: SnapshotStatusUrlParams::RepositorySnapshot(repository.into(),
                                                                    snapshot.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> SnapshotStatusRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            SnapshotStatusUrlParams::None => Cow::Borrowed("/_snapshot/_status"),
            SnapshotStatusUrlParams::Repository(ref repository) => {
                let mut url = String::with_capacity(19usize + repository.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                url.push_str("/_status");
                Cow::Owned(url)
            }
            SnapshotStatusUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
                let mut url = String::with_capacity(20usize + repository.len() + snapshot.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                url.push_str("/");
                url.push_str(snapshot.as_ref());
                url.push_str("/_status");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SnapshotStatusRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum MgetUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
pub struct MgetRequestParams<'a> {
    pub url_params: MgetUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> MgetRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> MgetRequestParams<'a> {
        MgetRequestParams {
            url_params: MgetUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> MgetRequestParams<'a> {
        MgetRequestParams {
            url_params: MgetUrlParams::Index(index.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> MgetRequestParams<'a> {
        MgetRequestParams {
            url_params: MgetUrlParams::IndexType(index.into(), ty.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> MgetRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            MgetUrlParams::None => Cow::Borrowed("/_mget"),
            MgetUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(7usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_mget");
                Cow::Owned(url)
            }
            MgetUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(8usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_mget");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a MgetRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum IndicesExistsTemplateUrlParams<'a> {
    Name(Name<'a>),
}
pub struct IndicesExistsTemplateRequestParams<'a> {
    pub url_params: IndicesExistsTemplateUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesExistsTemplateRequestParams<'a> {
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> IndicesExistsTemplateRequestParams<'a> {
        IndicesExistsTemplateRequestParams {
            url_params: IndicesExistsTemplateUrlParams::Name(name.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesExistsTemplateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesExistsTemplateUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(11usize + name.len());
                url.push_str("/_template/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesExistsTemplateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesGetUpgradeUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct IndicesGetUpgradeRequestParams<'a> {
    pub url_params: IndicesGetUpgradeUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesGetUpgradeRequestParams<'a> {
    pub fn new() -> IndicesGetUpgradeRequestParams<'a> {
        IndicesGetUpgradeRequestParams {
            url_params: IndicesGetUpgradeUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesGetUpgradeRequestParams<'a> {
        IndicesGetUpgradeRequestParams {
            url_params: IndicesGetUpgradeUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesGetUpgradeRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesGetUpgradeUrlParams::None => Cow::Borrowed("/_upgrade"),
            IndicesGetUpgradeUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(10usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_upgrade");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesGetUpgradeRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum PutScriptUrlParams<'a> {
    LangId(Lang<'a>, Id<'a>),
}
pub struct PutScriptRequestParams<'a> {
    pub url_params: PutScriptUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> PutScriptRequestParams<'a> {
    pub fn lang_id<ILang: Into<Lang<'a>>, IId: Into<Id<'a>>, IBody: Into<Body<'a>>>
        (lang: ILang,
         id: IId,
         body: IBody)
         -> PutScriptRequestParams<'a> {
        PutScriptRequestParams {
            url_params: PutScriptUrlParams::LangId(lang.into(), id.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> PutScriptRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            PutScriptUrlParams::LangId(ref lang, ref id) => {
                let mut url = String::with_capacity(11usize + lang.len() + id.len());
                url.push_str("/_scripts/");
                url.push_str(lang.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a PutScriptRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum GetTemplateUrlParams<'a> {
    Id(Id<'a>),
}
pub struct GetTemplateRequestParams<'a> {
    pub url_params: GetTemplateUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> GetTemplateRequestParams<'a> {
    pub fn id<IId: Into<Id<'a>>>(id: IId) -> GetTemplateRequestParams<'a> {
        GetTemplateRequestParams {
            url_params: GetTemplateUrlParams::Id(id.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> GetTemplateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            GetTemplateUrlParams::Id(ref id) => {
                let mut url = String::with_capacity(18usize + id.len());
                url.push_str("/_search/template/");
                url.push_str(id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a GetTemplateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesDeleteTemplateUrlParams<'a> {
    Name(Name<'a>),
}
pub struct IndicesDeleteTemplateRequestParams<'a> {
    pub url_params: IndicesDeleteTemplateUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesDeleteTemplateRequestParams<'a> {
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> IndicesDeleteTemplateRequestParams<'a> {
        IndicesDeleteTemplateRequestParams {
            url_params: IndicesDeleteTemplateUrlParams::Name(name.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesDeleteTemplateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesDeleteTemplateUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(11usize + name.len());
                url.push_str("/_template/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesDeleteTemplateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndexUrlParams<'a> {
    IndexType(Index<'a>, Type<'a>),
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
pub struct IndexRequestParams<'a> {
    pub url_params: IndexUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndexRequestParams<'a> {
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> IndexRequestParams<'a> {
        IndexRequestParams {
            url_params: IndexUrlParams::IndexType(index.into(), ty.into()),
            body: body.into(),
            _a: PhantomData,
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
            url_params: IndexUrlParams::IndexTypeId(index.into(), ty.into(), id.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> IndexRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndexUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(2usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                Cow::Owned(url)
            }
            IndexUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                let mut url = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndexRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum IndicesPutSettingsUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct IndicesPutSettingsRequestParams<'a> {
    pub url_params: IndicesPutSettingsUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesPutSettingsRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> IndicesPutSettingsRequestParams<'a> {
        IndicesPutSettingsRequestParams {
            url_params: IndicesPutSettingsUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> IndicesPutSettingsRequestParams<'a> {
        IndicesPutSettingsRequestParams {
            url_params: IndicesPutSettingsUrlParams::Index(index.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesPutSettingsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesPutSettingsUrlParams::None => Cow::Borrowed("/_settings"),
            IndicesPutSettingsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(11usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_settings");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesPutSettingsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum CatTemplatesUrlParams<'a> {
    None,
    Name(Name<'a>),
}
pub struct CatTemplatesRequestParams<'a> {
    pub url_params: CatTemplatesUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> CatTemplatesRequestParams<'a> {
    pub fn new() -> CatTemplatesRequestParams<'a> {
        CatTemplatesRequestParams {
            url_params: CatTemplatesUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> CatTemplatesRequestParams<'a> {
        CatTemplatesRequestParams {
            url_params: CatTemplatesUrlParams::Name(name.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> CatTemplatesRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CatTemplatesUrlParams::None => Cow::Borrowed("/_cat/templates"),
            CatTemplatesUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(16usize + name.len());
                url.push_str("/_cat/templates/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatTemplatesRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum CatIndicesUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct CatIndicesRequestParams<'a> {
    pub url_params: CatIndicesUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> CatIndicesRequestParams<'a> {
    pub fn new() -> CatIndicesRequestParams<'a> {
        CatIndicesRequestParams {
            url_params: CatIndicesUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> CatIndicesRequestParams<'a> {
        CatIndicesRequestParams {
            url_params: CatIndicesUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> CatIndicesRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CatIndicesUrlParams::None => Cow::Borrowed("/_cat/indices"),
            CatIndicesUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(14usize + index.len());
                url.push_str("/_cat/indices/");
                url.push_str(index.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatIndicesRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum ClusterPutSettingsUrlParams {
    None,
}
pub struct ClusterPutSettingsRequestParams<'a> {
    pub url_params: ClusterPutSettingsUrlParams,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> ClusterPutSettingsRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> ClusterPutSettingsRequestParams<'a> {
        ClusterPutSettingsRequestParams {
            url_params: ClusterPutSettingsUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> ClusterPutSettingsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ClusterPutSettingsUrlParams::None => Cow::Borrowed("/_cluster/settings"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ClusterPutSettingsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum UpdateUrlParams<'a> {
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
pub struct UpdateRequestParams<'a> {
    pub url_params: UpdateUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
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
            url_params: UpdateUrlParams::IndexTypeId(index.into(), ty.into(), id.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> UpdateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            UpdateUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                let mut url = String::with_capacity(11usize + index.len() + ty.len() + id.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/");
                url.push_str(id.as_ref());
                url.push_str("/_update");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a UpdateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum IndicesPutAliasUrlParams<'a> {
    IndexName(Index<'a>, Name<'a>),
}
pub struct IndicesPutAliasRequestParams<'a> {
    pub url_params: IndicesPutAliasUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesPutAliasRequestParams<'a> {
    pub fn index_name<IIndex: Into<Index<'a>>, IName: Into<Name<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         name: IName,
         body: IBody)
         -> IndicesPutAliasRequestParams<'a> {
        IndicesPutAliasRequestParams {
            url_params: IndicesPutAliasUrlParams::IndexName(index.into(), name.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesPutAliasRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesPutAliasUrlParams::IndexName(ref index, ref name) => {
                let mut url = String::with_capacity(11usize + index.len() + name.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_aliases/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesPutAliasRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum CatPluginsUrlParams {
    None,
}
pub struct CatPluginsRequestParams<'a> {
    pub url_params: CatPluginsUrlParams,
    _a: PhantomData<&'a ()>,
}
impl<'a> CatPluginsRequestParams<'a> {
    pub fn new() -> CatPluginsRequestParams<'a> {
        CatPluginsRequestParams {
            url_params: CatPluginsUrlParams::None,
            _a: PhantomData,
        }
    }
}
impl<'a> CatPluginsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CatPluginsUrlParams::None => Cow::Borrowed("/_cat/plugins"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatPluginsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum CountPercolateUrlParams<'a> {
    IndexType(Index<'a>, Type<'a>),
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
pub struct CountPercolateRequestParams<'a> {
    pub url_params: CountPercolateUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> CountPercolateRequestParams<'a> {
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> CountPercolateRequestParams<'a> {
        CountPercolateRequestParams {
            url_params: CountPercolateUrlParams::IndexType(index.into(), ty.into()),
            body: body.into(),
            _a: PhantomData,
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
            url_params: CountPercolateUrlParams::IndexTypeId(index.into(), ty.into(), id.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> CountPercolateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CountPercolateUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(19usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_percolate/count");
                Cow::Owned(url)
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
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CountPercolateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum IndicesUpgradeUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct IndicesUpgradeRequestParams<'a> {
    pub url_params: IndicesUpgradeUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesUpgradeRequestParams<'a> {
    pub fn new() -> IndicesUpgradeRequestParams<'a> {
        IndicesUpgradeRequestParams {
            url_params: IndicesUpgradeUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesUpgradeRequestParams<'a> {
        IndicesUpgradeRequestParams {
            url_params: IndicesUpgradeUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesUpgradeRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesUpgradeUrlParams::None => Cow::Borrowed("/_upgrade"),
            IndicesUpgradeUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(10usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_upgrade");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesUpgradeRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesDeleteAliasUrlParams<'a> {
    IndexName(Index<'a>, Name<'a>),
}
pub struct IndicesDeleteAliasRequestParams<'a> {
    pub url_params: IndicesDeleteAliasUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesDeleteAliasRequestParams<'a> {
    pub fn index_name<IIndex: Into<Index<'a>>, IName: Into<Name<'a>>>
        (index: IIndex,
         name: IName)
         -> IndicesDeleteAliasRequestParams<'a> {
        IndicesDeleteAliasRequestParams {
            url_params: IndicesDeleteAliasUrlParams::IndexName(index.into(), name.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesDeleteAliasRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesDeleteAliasUrlParams::IndexName(ref index, ref name) => {
                let mut url = String::with_capacity(11usize + index.len() + name.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_aliases/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesDeleteAliasRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum CatTasksUrlParams {
    None,
}
pub struct CatTasksRequestParams<'a> {
    pub url_params: CatTasksUrlParams,
    _a: PhantomData<&'a ()>,
}
impl<'a> CatTasksRequestParams<'a> {
    pub fn new() -> CatTasksRequestParams<'a> {
        CatTasksRequestParams {
            url_params: CatTasksUrlParams::None,
            _a: PhantomData,
        }
    }
}
impl<'a> CatTasksRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CatTasksUrlParams::None => Cow::Borrowed("/_cat/tasks"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatTasksRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesRolloverUrlParams<'a> {
    Alias(Alias<'a>),
    AliasNewIndex(Alias<'a>, NewIndex<'a>),
}
pub struct IndicesRolloverRequestParams<'a> {
    pub url_params: IndicesRolloverUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesRolloverRequestParams<'a> {
    pub fn alias<IAlias: Into<Alias<'a>>, IBody: Into<Body<'a>>>
        (alias: IAlias,
         body: IBody)
         -> IndicesRolloverRequestParams<'a> {
        IndicesRolloverRequestParams {
            url_params: IndicesRolloverUrlParams::Alias(alias.into()),
            body: body.into(),
            _a: PhantomData,
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
            url_params: IndicesRolloverUrlParams::AliasNewIndex(alias.into(), new_index.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesRolloverRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesRolloverUrlParams::Alias(ref alias) => {
                let mut url = String::with_capacity(11usize + alias.len());
                url.push_str("/");
                url.push_str(alias.as_ref());
                url.push_str("/_rollover");
                Cow::Owned(url)
            }
            IndicesRolloverUrlParams::AliasNewIndex(ref alias, ref new_index) => {
                let mut url = String::with_capacity(12usize + alias.len() + new_index.len());
                url.push_str("/");
                url.push_str(alias.as_ref());
                url.push_str("/_rollover/");
                url.push_str(new_index.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesRolloverRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum ReindexRethrottleUrlParams<'a> {
    TaskId(TaskId<'a>),
}
pub struct ReindexRethrottleRequestParams<'a> {
    pub url_params: ReindexRethrottleUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> ReindexRethrottleRequestParams<'a> {
    pub fn task_id<ITaskId: Into<TaskId<'a>>>(task_id: ITaskId)
                                              -> ReindexRethrottleRequestParams<'a> {
        ReindexRethrottleRequestParams {
            url_params: ReindexRethrottleUrlParams::TaskId(task_id.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> ReindexRethrottleRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ReindexRethrottleUrlParams::TaskId(ref task_id) => {
                let mut url = String::with_capacity(30usize + task_id.len());
                url.push_str("/_delete_by_query/");
                url.push_str(task_id.as_ref());
                url.push_str("/_rethrottle");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ReindexRethrottleRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum SnapshotCreateRepositoryUrlParams<'a> {
    Repository(Repository<'a>),
}
pub struct SnapshotCreateRepositoryRequestParams<'a> {
    pub url_params: SnapshotCreateRepositoryUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> SnapshotCreateRepositoryRequestParams<'a> {
    pub fn repository<IRepository: Into<Repository<'a>>, IBody: Into<Body<'a>>>
        (repository: IRepository,
         body: IBody)
         -> SnapshotCreateRepositoryRequestParams<'a> {
        SnapshotCreateRepositoryRequestParams {
            url_params: SnapshotCreateRepositoryUrlParams::Repository(repository.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> SnapshotCreateRepositoryRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            SnapshotCreateRepositoryUrlParams::Repository(ref repository) => {
                let mut url = String::with_capacity(11usize + repository.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SnapshotCreateRepositoryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum IndicesGetUrlParams<'a> {
    Index(Index<'a>),
    IndexFeature(Index<'a>, Feature<'a>),
}
pub struct IndicesGetRequestParams<'a> {
    pub url_params: IndicesGetUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesGetRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesGetRequestParams<'a> {
        IndicesGetRequestParams {
            url_params: IndicesGetUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
    pub fn index_feature<IIndex: Into<Index<'a>>, IFeature: Into<Feature<'a>>>
        (index: IIndex,
         feature: IFeature)
         -> IndicesGetRequestParams<'a> {
        IndicesGetRequestParams {
            url_params: IndicesGetUrlParams::IndexFeature(index.into(), feature.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesGetRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesGetUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(1usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                Cow::Owned(url)
            }
            IndicesGetUrlParams::IndexFeature(ref index, ref feature) => {
                let mut url = String::with_capacity(2usize + index.len() + feature.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(feature.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesGetRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesAnalyzeUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct IndicesAnalyzeRequestParams<'a> {
    pub url_params: IndicesAnalyzeUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesAnalyzeRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> IndicesAnalyzeRequestParams<'a> {
        IndicesAnalyzeRequestParams {
            url_params: IndicesAnalyzeUrlParams::None,
            body: body.into(),
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> IndicesAnalyzeRequestParams<'a> {
        IndicesAnalyzeRequestParams {
            url_params: IndicesAnalyzeUrlParams::Index(index.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesAnalyzeRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesAnalyzeUrlParams::None => Cow::Borrowed("/_analyze"),
            IndicesAnalyzeUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(10usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_analyze");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesAnalyzeRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum CatFielddataUrlParams<'a> {
    None,
    Fields(Fields<'a>),
}
pub struct CatFielddataRequestParams<'a> {
    pub url_params: CatFielddataUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> CatFielddataRequestParams<'a> {
    pub fn new() -> CatFielddataRequestParams<'a> {
        CatFielddataRequestParams {
            url_params: CatFielddataUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn fields<IFields: Into<Fields<'a>>>(fields: IFields) -> CatFielddataRequestParams<'a> {
        CatFielddataRequestParams {
            url_params: CatFielddataUrlParams::Fields(fields.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> CatFielddataRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CatFielddataUrlParams::None => Cow::Borrowed("/_cat/fielddata"),
            CatFielddataUrlParams::Fields(ref fields) => {
                let mut url = String::with_capacity(16usize + fields.len());
                url.push_str("/_cat/fielddata/");
                url.push_str(fields.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatFielddataRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesSegmentsUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct IndicesSegmentsRequestParams<'a> {
    pub url_params: IndicesSegmentsUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesSegmentsRequestParams<'a> {
    pub fn new() -> IndicesSegmentsRequestParams<'a> {
        IndicesSegmentsRequestParams {
            url_params: IndicesSegmentsUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesSegmentsRequestParams<'a> {
        IndicesSegmentsRequestParams {
            url_params: IndicesSegmentsUrlParams::Index(index.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesSegmentsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesSegmentsUrlParams::None => Cow::Borrowed("/_segments"),
            IndicesSegmentsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(11usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_segments");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesSegmentsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesShrinkUrlParams<'a> {
    IndexTarget(Index<'a>, Target<'a>),
}
pub struct IndicesShrinkRequestParams<'a> {
    pub url_params: IndicesShrinkUrlParams<'a>,
    pub body: Body<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesShrinkRequestParams<'a> {
    pub fn index_target<IIndex: Into<Index<'a>>, ITarget: Into<Target<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         target: ITarget,
         body: IBody)
         -> IndicesShrinkRequestParams<'a> {
        IndicesShrinkRequestParams {
            url_params: IndicesShrinkUrlParams::IndexTarget(index.into(), target.into()),
            body: body.into(),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesShrinkRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesShrinkUrlParams::IndexTarget(ref index, ref target) => {
                let mut url = String::with_capacity(10usize + index.len() + target.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_shrink/");
                url.push_str(target.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesShrinkRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum TasksListUrlParams {
    None,
}
pub struct TasksListRequestParams<'a> {
    pub url_params: TasksListUrlParams,
    _a: PhantomData<&'a ()>,
}
impl<'a> TasksListRequestParams<'a> {
    pub fn new() -> TasksListRequestParams<'a> {
        TasksListRequestParams {
            url_params: TasksListUrlParams::None,
            _a: PhantomData,
        }
    }
}
impl<'a> TasksListRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            TasksListUrlParams::None => Cow::Borrowed("/_tasks"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a TasksListRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum CatMasterUrlParams {
    None,
}
pub struct CatMasterRequestParams<'a> {
    pub url_params: CatMasterUrlParams,
    _a: PhantomData<&'a ()>,
}
impl<'a> CatMasterRequestParams<'a> {
    pub fn new() -> CatMasterRequestParams<'a> {
        CatMasterRequestParams {
            url_params: CatMasterUrlParams::None,
            _a: PhantomData,
        }
    }
}
impl<'a> CatMasterRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CatMasterUrlParams::None => Cow::Borrowed("/_cat/master"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CatMasterRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesExistsTypeUrlParams<'a> {
    IndexType(Index<'a>, Type<'a>),
}
pub struct IndicesExistsTypeRequestParams<'a> {
    pub url_params: IndicesExistsTypeUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> IndicesExistsTypeRequestParams<'a> {
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>>
        (index: IIndex,
         ty: IType)
         -> IndicesExistsTypeRequestParams<'a> {
        IndicesExistsTypeRequestParams {
            url_params: IndicesExistsTypeUrlParams::IndexType(index.into(), ty.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> IndicesExistsTypeRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesExistsTypeUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(11usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_mapping/");
                url.push_str(ty.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesExistsTypeRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum ClusterGetSettingsUrlParams {
    None,
}
pub struct ClusterGetSettingsRequestParams<'a> {
    pub url_params: ClusterGetSettingsUrlParams,
    _a: PhantomData<&'a ()>,
}
impl<'a> ClusterGetSettingsRequestParams<'a> {
    pub fn new() -> ClusterGetSettingsRequestParams<'a> {
        ClusterGetSettingsRequestParams {
            url_params: ClusterGetSettingsUrlParams::None,
            _a: PhantomData,
        }
    }
}
impl<'a> ClusterGetSettingsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ClusterGetSettingsUrlParams::None => Cow::Borrowed("/_cluster/settings"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ClusterGetSettingsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum NodesInfoUrlParams<'a> {
    None,
    Metric(Metric<'a>),
    NodeId(NodeId<'a>),
    NodeIdMetric(NodeId<'a>, Metric<'a>),
}
pub struct NodesInfoRequestParams<'a> {
    pub url_params: NodesInfoUrlParams<'a>,
    _a: PhantomData<&'a ()>,
}
impl<'a> NodesInfoRequestParams<'a> {
    pub fn new() -> NodesInfoRequestParams<'a> {
        NodesInfoRequestParams {
            url_params: NodesInfoUrlParams::None,
            _a: PhantomData,
        }
    }
    pub fn metric<IMetric: Into<Metric<'a>>>(metric: IMetric) -> NodesInfoRequestParams<'a> {
        NodesInfoRequestParams {
            url_params: NodesInfoUrlParams::Metric(metric.into()),
            _a: PhantomData,
        }
    }
    pub fn node_id<INodeId: Into<NodeId<'a>>>(node_id: INodeId) -> NodesInfoRequestParams<'a> {
        NodesInfoRequestParams {
            url_params: NodesInfoUrlParams::NodeId(node_id.into()),
            _a: PhantomData,
        }
    }
    pub fn node_id_metric<INodeId: Into<NodeId<'a>>, IMetric: Into<Metric<'a>>>
        (node_id: INodeId,
         metric: IMetric)
         -> NodesInfoRequestParams<'a> {
        NodesInfoRequestParams {
            url_params: NodesInfoUrlParams::NodeIdMetric(node_id.into(), metric.into()),
            _a: PhantomData,
        }
    }
}
impl<'a> NodesInfoRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            NodesInfoUrlParams::None => Cow::Borrowed("/_nodes"),
            NodesInfoUrlParams::Metric(ref metric) => {
                let mut url = String::with_capacity(8usize + metric.len());
                url.push_str("/_nodes/");
                url.push_str(metric.as_ref());
                Cow::Owned(url)
            }
            NodesInfoUrlParams::NodeId(ref node_id) => {
                let mut url = String::with_capacity(8usize + node_id.len());
                url.push_str("/_nodes/");
                url.push_str(node_id.as_ref());
                Cow::Owned(url)
            }
            NodesInfoUrlParams::NodeIdMetric(ref node_id, ref metric) => {
                let mut url = String::with_capacity(9usize + node_id.len() + metric.len());
                url.push_str("/_nodes/");
                url.push_str(node_id.as_ref());
                url.push_str("/");
                url.push_str(metric.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a NodesInfoRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

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
