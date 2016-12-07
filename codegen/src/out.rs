// This code is automatically generated
//
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
    pub body: Option<Body<'a>>,
}
pub enum HttpMethod {
    Head,
    Get,
    Post,
    Put,
    Delete,
}

pub enum CloseUrlParams<'a> {
    Index(Index<'a>),
}
pub struct CloseRequestParams<'a> {
    pub url_params: CloseUrlParams<'a>,
}
impl<'a> CloseRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> CloseRequestParams<'a> {
        CloseRequestParams { url_params: CloseUrlParams::Index(index.into()) }
    }
}
impl<'a> CloseRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CloseUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(8usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_close");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CloseRequestParams<'b> {
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
}
impl<'a> DeleteScriptRequestParams<'a> {
    pub fn lang_id<ILang: Into<Lang<'a>>, IId: Into<Id<'a>>>(lang: ILang,
                                                             id: IId)
                                                             -> DeleteScriptRequestParams<'a> {
        DeleteScriptRequestParams {
            url_params: DeleteScriptUrlParams::LangId(lang.into(), id.into()),
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
}
impl<'a> FieldStatsRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> FieldStatsRequestParams<'a> {
        FieldStatsRequestParams {
            url_params: FieldStatsUrlParams::None,
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> FieldStatsRequestParams<'a> {
        FieldStatsRequestParams {
            url_params: FieldStatsUrlParams::Index(index.into()),
            body: body.into(),
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

pub enum ThreadPoolUrlParams<'a> {
    None,
    ThreadPoolPatterns(ThreadPoolPatterns<'a>),
}
pub struct ThreadPoolRequestParams<'a> {
    pub url_params: ThreadPoolUrlParams<'a>,
}
impl<'a> ThreadPoolRequestParams<'a> {
    pub fn new() -> ThreadPoolRequestParams<'a> {
        ThreadPoolRequestParams { url_params: ThreadPoolUrlParams::None }
    }
    pub fn thread_pool_patterns<IThreadPoolPatterns: Into<ThreadPoolPatterns<'a>>>
        (thread_pool_patterns: IThreadPoolPatterns)
         -> ThreadPoolRequestParams<'a> {
        ThreadPoolRequestParams {
            url_params: ThreadPoolUrlParams::ThreadPoolPatterns(thread_pool_patterns.into()),
        }
    }
}
impl<'a> ThreadPoolRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ThreadPoolUrlParams::None => Cow::Borrowed("/_cat/thread_pool"),
            ThreadPoolUrlParams::ThreadPoolPatterns(ref thread_pool_patterns) => {
                let mut url = String::with_capacity(18usize + thread_pool_patterns.len());
                url.push_str("/_cat/thread_pool/");
                url.push_str(thread_pool_patterns.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ThreadPoolRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum DeleteUrlParams<'a> {
    RepositorySnapshot(Repository<'a>, Snapshot<'a>),
}
pub struct DeleteRequestParams<'a> {
    pub url_params: DeleteUrlParams<'a>,
}
impl<'a> DeleteRequestParams<'a> {
    pub fn repository_snapshot<IRepository: Into<Repository<'a>>, ISnapshot: Into<Snapshot<'a>>>
        (repository: IRepository,
         snapshot: ISnapshot)
         -> DeleteRequestParams<'a> {
        DeleteRequestParams {
            url_params: DeleteUrlParams::RepositorySnapshot(repository.into(), snapshot.into()),
        }
    }
}
impl<'a> DeleteRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            DeleteUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
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
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a DeleteRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum GetSettingsUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexName(Index<'a>, Name<'a>),
    Name(Name<'a>),
}
pub struct GetSettingsRequestParams<'a> {
    pub url_params: GetSettingsUrlParams<'a>,
}
impl<'a> GetSettingsRequestParams<'a> {
    pub fn new() -> GetSettingsRequestParams<'a> {
        GetSettingsRequestParams { url_params: GetSettingsUrlParams::None }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> GetSettingsRequestParams<'a> {
        GetSettingsRequestParams { url_params: GetSettingsUrlParams::Index(index.into()) }
    }
    pub fn index_name<IIndex: Into<Index<'a>>, IName: Into<Name<'a>>>
        (index: IIndex,
         name: IName)
         -> GetSettingsRequestParams<'a> {
        GetSettingsRequestParams {
            url_params: GetSettingsUrlParams::IndexName(index.into(), name.into()),
        }
    }
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> GetSettingsRequestParams<'a> {
        GetSettingsRequestParams { url_params: GetSettingsUrlParams::Name(name.into()) }
    }
}
impl<'a> GetSettingsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            GetSettingsUrlParams::None => Cow::Borrowed("/_settings"),
            GetSettingsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(11usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_settings");
                Cow::Owned(url)
            }
            GetSettingsUrlParams::IndexName(ref index, ref name) => {
                let mut url = String::with_capacity(12usize + index.len() + name.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_settings/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
            GetSettingsUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(11usize + name.len());
                url.push_str("/_settings/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a GetSettingsRequestParams<'b> {
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

pub enum DeleteRepositoryUrlParams<'a> {
    Repository(Repository<'a>),
}
pub struct DeleteRepositoryRequestParams<'a> {
    pub url_params: DeleteRepositoryUrlParams<'a>,
}
impl<'a> DeleteRepositoryRequestParams<'a> {
    pub fn repository<IRepository: Into<Repository<'a>>>(repository: IRepository)
                                                         -> DeleteRepositoryRequestParams<'a> {
        DeleteRepositoryRequestParams {
            url_params: DeleteRepositoryUrlParams::Repository(repository.into()),
        }
    }
}
impl<'a> DeleteRepositoryRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            DeleteRepositoryUrlParams::Repository(ref repository) => {
                let mut url = String::with_capacity(11usize + repository.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a DeleteRepositoryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum AllocationExplainUrlParams<'a> {
    None,
}
pub struct AllocationExplainRequestParams<'a> {
    pub url_params: AllocationExplainUrlParams<'a>,
    pub body: Body<'a>,
}
impl<'a> AllocationExplainRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> AllocationExplainRequestParams<'a> {
        AllocationExplainRequestParams {
            url_params: AllocationExplainUrlParams::None,
            body: body.into(),
        }
    }
}
impl<'a> AllocationExplainRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            AllocationExplainUrlParams::None => Cow::Borrowed("/_cluster/allocation/explain"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a AllocationExplainRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum PutTemplateUrlParams<'a> {
    Name(Name<'a>),
}
pub struct PutTemplateRequestParams<'a> {
    pub url_params: PutTemplateUrlParams<'a>,
    pub body: Body<'a>,
}
impl<'a> PutTemplateRequestParams<'a> {
    pub fn name<IName: Into<Name<'a>>, IBody: Into<Body<'a>>>(name: IName,
                                                              body: IBody)
                                                              -> PutTemplateRequestParams<'a> {
        PutTemplateRequestParams {
            url_params: PutTemplateUrlParams::Name(name.into()),
            body: body.into(),
        }
    }
}
impl<'a> PutTemplateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            PutTemplateUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(11usize + name.len());
                url.push_str("/_template/");
                url.push_str(name.as_ref());
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

pub enum GetTemplateUrlParams<'a> {
    None,
    Name(Name<'a>),
}
pub struct GetTemplateRequestParams<'a> {
    pub url_params: GetTemplateUrlParams<'a>,
}
impl<'a> GetTemplateRequestParams<'a> {
    pub fn new() -> GetTemplateRequestParams<'a> {
        GetTemplateRequestParams { url_params: GetTemplateUrlParams::None }
    }
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> GetTemplateRequestParams<'a> {
        GetTemplateRequestParams { url_params: GetTemplateUrlParams::Name(name.into()) }
    }
}
impl<'a> GetTemplateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            GetTemplateUrlParams::None => Cow::Borrowed("/_template"),
            GetTemplateUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(11usize + name.len());
                url.push_str("/_template/");
                url.push_str(name.as_ref());
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

pub enum StateUrlParams<'a> {
    None,
    Metric(Metric<'a>),
    MetricIndex(Metric<'a>, Index<'a>),
}
pub struct StateRequestParams<'a> {
    pub url_params: StateUrlParams<'a>,
}
impl<'a> StateRequestParams<'a> {
    pub fn new() -> StateRequestParams<'a> {
        StateRequestParams { url_params: StateUrlParams::None }
    }
    pub fn metric<IMetric: Into<Metric<'a>>>(metric: IMetric) -> StateRequestParams<'a> {
        StateRequestParams { url_params: StateUrlParams::Metric(metric.into()) }
    }
    pub fn metric_index<IMetric: Into<Metric<'a>>, IIndex: Into<Index<'a>>>
        (metric: IMetric,
         index: IIndex)
         -> StateRequestParams<'a> {
        StateRequestParams { url_params: StateUrlParams::MetricIndex(metric.into(), index.into()) }
    }
}
impl<'a> StateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            StateUrlParams::None => Cow::Borrowed("/_cluster/state"),
            StateUrlParams::Metric(ref metric) => {
                let mut url = String::with_capacity(16usize + metric.len());
                url.push_str("/_cluster/state/");
                url.push_str(metric.as_ref());
                Cow::Owned(url)
            }
            StateUrlParams::MetricIndex(ref metric, ref index) => {
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
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a StateRequestParams<'b> {
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
}
impl<'a> MsearchTemplateRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> MsearchTemplateRequestParams<'a> {
        MsearchTemplateRequestParams {
            url_params: MsearchTemplateUrlParams::None,
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> MsearchTemplateRequestParams<'a> {
        MsearchTemplateRequestParams {
            url_params: MsearchTemplateUrlParams::Index(index.into()),
            body: body.into(),
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
}
impl<'a> BulkRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> BulkRequestParams<'a> {
        BulkRequestParams {
            url_params: BulkUrlParams::None,
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> BulkRequestParams<'a> {
        BulkRequestParams {
            url_params: BulkUrlParams::Index(index.into()),
            body: body.into(),
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
}
impl<'a> SuggestRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> SuggestRequestParams<'a> {
        SuggestRequestParams {
            url_params: SuggestUrlParams::None,
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> SuggestRequestParams<'a> {
        SuggestRequestParams {
            url_params: SuggestUrlParams::Index(index.into()),
            body: body.into(),
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

pub enum GetRepositoryUrlParams<'a> {
    None,
    Repository(Repository<'a>),
}
pub struct GetRepositoryRequestParams<'a> {
    pub url_params: GetRepositoryUrlParams<'a>,
}
impl<'a> GetRepositoryRequestParams<'a> {
    pub fn new() -> GetRepositoryRequestParams<'a> {
        GetRepositoryRequestParams { url_params: GetRepositoryUrlParams::None }
    }
    pub fn repository<IRepository: Into<Repository<'a>>>(repository: IRepository)
                                                         -> GetRepositoryRequestParams<'a> {
        GetRepositoryRequestParams {
            url_params: GetRepositoryUrlParams::Repository(repository.into()),
        }
    }
}
impl<'a> GetRepositoryRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            GetRepositoryUrlParams::None => Cow::Borrowed("/_snapshot"),
            GetRepositoryUrlParams::Repository(ref repository) => {
                let mut url = String::with_capacity(11usize + repository.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a GetRepositoryRequestParams<'b> {
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
}
impl<'a> RenderSearchTemplateRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> RenderSearchTemplateRequestParams<'a> {
        RenderSearchTemplateRequestParams {
            url_params: RenderSearchTemplateUrlParams::None,
            body: body.into(),
        }
    }
    pub fn id<IId: Into<Id<'a>>, IBody: Into<Body<'a>>>
        (id: IId,
         body: IBody)
         -> RenderSearchTemplateRequestParams<'a> {
        RenderSearchTemplateRequestParams {
            url_params: RenderSearchTemplateUrlParams::Id(id.into()),
            body: body.into(),
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

pub enum StatsUrlParams<'a> {
    None,
    Metric(Metric<'a>),
    Index(Index<'a>),
    IndexMetric(Index<'a>, Metric<'a>),
}
pub struct StatsRequestParams<'a> {
    pub url_params: StatsUrlParams<'a>,
}
impl<'a> StatsRequestParams<'a> {
    pub fn new() -> StatsRequestParams<'a> {
        StatsRequestParams { url_params: StatsUrlParams::None }
    }
    pub fn metric<IMetric: Into<Metric<'a>>>(metric: IMetric) -> StatsRequestParams<'a> {
        StatsRequestParams { url_params: StatsUrlParams::Metric(metric.into()) }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> StatsRequestParams<'a> {
        StatsRequestParams { url_params: StatsUrlParams::Index(index.into()) }
    }
    pub fn index_metric<IIndex: Into<Index<'a>>, IMetric: Into<Metric<'a>>>
        (index: IIndex,
         metric: IMetric)
         -> StatsRequestParams<'a> {
        StatsRequestParams { url_params: StatsUrlParams::IndexMetric(index.into(), metric.into()) }
    }
}
impl<'a> StatsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            StatsUrlParams::None => Cow::Borrowed("/_stats"),
            StatsUrlParams::Metric(ref metric) => {
                let mut url = String::with_capacity(8usize + metric.len());
                url.push_str("/_stats/");
                url.push_str(metric.as_ref());
                Cow::Owned(url)
            }
            StatsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(8usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_stats");
                Cow::Owned(url)
            }
            StatsUrlParams::IndexMetric(ref index, ref metric) => {
                let mut url = String::with_capacity(9usize + index.len() + metric.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_stats/");
                url.push_str(metric.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a StatsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum RepositoriesUrlParams<'a> {
    None,
}
pub struct RepositoriesRequestParams<'a> {
    pub url_params: RepositoriesUrlParams<'a>,
}
impl<'a> RepositoriesRequestParams<'a> {
    pub fn new() -> RepositoriesRequestParams<'a> {
        RepositoriesRequestParams { url_params: RepositoriesUrlParams::None }
    }
}
impl<'a> RepositoriesRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            RepositoriesUrlParams::None => Cow::Borrowed("/_cat/repositories"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a RepositoriesRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum ForcemergeUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct ForcemergeRequestParams<'a> {
    pub url_params: ForcemergeUrlParams<'a>,
}
impl<'a> ForcemergeRequestParams<'a> {
    pub fn new() -> ForcemergeRequestParams<'a> {
        ForcemergeRequestParams { url_params: ForcemergeUrlParams::None }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> ForcemergeRequestParams<'a> {
        ForcemergeRequestParams { url_params: ForcemergeUrlParams::Index(index.into()) }
    }
}
impl<'a> ForcemergeRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ForcemergeUrlParams::None => Cow::Borrowed("/_forcemerge"),
            ForcemergeUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(13usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_forcemerge");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ForcemergeRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum PingUrlParams<'a> {
    None,
}
pub struct PingRequestParams<'a> {
    pub url_params: PingUrlParams<'a>,
}
impl<'a> PingRequestParams<'a> {
    pub fn new() -> PingRequestParams<'a> {
        PingRequestParams { url_params: PingUrlParams::None }
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

pub enum GetUrlParams<'a> {
    TaskId(TaskId<'a>),
}
pub struct GetRequestParams<'a> {
    pub url_params: GetUrlParams<'a>,
}
impl<'a> GetRequestParams<'a> {
    pub fn task_id<ITaskId: Into<TaskId<'a>>>(task_id: ITaskId) -> GetRequestParams<'a> {
        GetRequestParams { url_params: GetUrlParams::TaskId(task_id.into()) }
    }
}
impl<'a> GetRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            GetUrlParams::TaskId(ref task_id) => {
                let mut url = String::with_capacity(8usize + task_id.len());
                url.push_str("/_tasks/");
                url.push_str(task_id.as_ref());
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

pub enum ExistsUrlParams<'a> {
    Index(Index<'a>),
}
pub struct ExistsRequestParams<'a> {
    pub url_params: ExistsUrlParams<'a>,
}
impl<'a> ExistsRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> ExistsRequestParams<'a> {
        ExistsRequestParams { url_params: ExistsUrlParams::Index(index.into()) }
    }
}
impl<'a> ExistsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ExistsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(1usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
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

pub enum FlushSyncedUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct FlushSyncedRequestParams<'a> {
    pub url_params: FlushSyncedUrlParams<'a>,
}
impl<'a> FlushSyncedRequestParams<'a> {
    pub fn new() -> FlushSyncedRequestParams<'a> {
        FlushSyncedRequestParams { url_params: FlushSyncedUrlParams::None }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> FlushSyncedRequestParams<'a> {
        FlushSyncedRequestParams { url_params: FlushSyncedUrlParams::Index(index.into()) }
    }
}
impl<'a> FlushSyncedRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            FlushSyncedUrlParams::None => Cow::Borrowed("/_flush/synced"),
            FlushSyncedUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(15usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_flush/synced");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a FlushSyncedRequestParams<'b> {
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
}
impl<'a> MsearchRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> MsearchRequestParams<'a> {
        MsearchRequestParams {
            url_params: MsearchUrlParams::None,
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> MsearchRequestParams<'a> {
        MsearchRequestParams {
            url_params: MsearchUrlParams::Index(index.into()),
            body: body.into(),
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

pub enum InfoUrlParams<'a> {
    None,
}
pub struct InfoRequestParams<'a> {
    pub url_params: InfoUrlParams<'a>,
}
impl<'a> InfoRequestParams<'a> {
    pub fn new() -> InfoRequestParams<'a> {
        InfoRequestParams { url_params: InfoUrlParams::None }
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
}
impl<'a> SearchTemplateRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> SearchTemplateRequestParams<'a> {
        SearchTemplateRequestParams {
            url_params: SearchTemplateUrlParams::None,
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> SearchTemplateRequestParams<'a> {
        SearchTemplateRequestParams {
            url_params: SearchTemplateUrlParams::Index(index.into()),
            body: body.into(),
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

pub enum DeleteUrlParams<'a> {
    Index(Index<'a>),
}
pub struct DeleteRequestParams<'a> {
    pub url_params: DeleteUrlParams<'a>,
}
impl<'a> DeleteRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> DeleteRequestParams<'a> {
        DeleteRequestParams { url_params: DeleteUrlParams::Index(index.into()) }
    }
}
impl<'a> DeleteRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            DeleteUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(1usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
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

pub enum DeleteByQueryUrlParams<'a> {
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
pub struct DeleteByQueryRequestParams<'a> {
    pub url_params: DeleteByQueryUrlParams<'a>,
    pub body: Body<'a>,
}
impl<'a> DeleteByQueryRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> DeleteByQueryRequestParams<'a> {
        DeleteByQueryRequestParams {
            url_params: DeleteByQueryUrlParams::Index(index.into()),
            body: body.into(),
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
}
impl<'a> DeleteTemplateRequestParams<'a> {
    pub fn id<IId: Into<Id<'a>>>(id: IId) -> DeleteTemplateRequestParams<'a> {
        DeleteTemplateRequestParams { url_params: DeleteTemplateUrlParams::Id(id.into()) }
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

pub enum CreateUrlParams<'a> {
    Index(Index<'a>),
}
pub struct CreateRequestParams<'a> {
    pub url_params: CreateUrlParams<'a>,
    pub body: Body<'a>,
}
impl<'a> CreateRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> CreateRequestParams<'a> {
        CreateRequestParams {
            url_params: CreateUrlParams::Index(index.into()),
            body: body.into(),
        }
    }
}
impl<'a> CreateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CreateUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(1usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
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

pub enum PercolateUrlParams<'a> {
    IndexType(Index<'a>, Type<'a>),
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
pub struct PercolateRequestParams<'a> {
    pub url_params: PercolateUrlParams<'a>,
    pub body: Body<'a>,
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
}
impl<'a> SearchRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> SearchRequestParams<'a> {
        SearchRequestParams {
            url_params: SearchUrlParams::None,
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> SearchRequestParams<'a> {
        SearchRequestParams {
            url_params: SearchUrlParams::Index(index.into()),
            body: body.into(),
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

pub enum NodeattrsUrlParams<'a> {
    None,
}
pub struct NodeattrsRequestParams<'a> {
    pub url_params: NodeattrsUrlParams<'a>,
}
impl<'a> NodeattrsRequestParams<'a> {
    pub fn new() -> NodeattrsRequestParams<'a> {
        NodeattrsRequestParams { url_params: NodeattrsUrlParams::None }
    }
}
impl<'a> NodeattrsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            NodeattrsUrlParams::None => Cow::Borrowed("/_cat/nodeattrs"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a NodeattrsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum VerifyRepositoryUrlParams<'a> {
    Repository(Repository<'a>),
}
pub struct VerifyRepositoryRequestParams<'a> {
    pub url_params: VerifyRepositoryUrlParams<'a>,
}
impl<'a> VerifyRepositoryRequestParams<'a> {
    pub fn repository<IRepository: Into<Repository<'a>>>(repository: IRepository)
                                                         -> VerifyRepositoryRequestParams<'a> {
        VerifyRepositoryRequestParams {
            url_params: VerifyRepositoryUrlParams::Repository(repository.into()),
        }
    }
}
impl<'a> VerifyRepositoryRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            VerifyRepositoryUrlParams::Repository(ref repository) => {
                let mut url = String::with_capacity(19usize + repository.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                url.push_str("/_verify");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a VerifyRepositoryRequestParams<'b> {
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
}
impl<'a> CountRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> CountRequestParams<'a> {
        CountRequestParams {
            url_params: CountUrlParams::None,
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> CountRequestParams<'a> {
        CountRequestParams {
            url_params: CountUrlParams::Index(index.into()),
            body: body.into(),
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

pub enum AllocationUrlParams<'a> {
    None,
    NodeId(NodeId<'a>),
}
pub struct AllocationRequestParams<'a> {
    pub url_params: AllocationUrlParams<'a>,
}
impl<'a> AllocationRequestParams<'a> {
    pub fn new() -> AllocationRequestParams<'a> {
        AllocationRequestParams { url_params: AllocationUrlParams::None }
    }
    pub fn node_id<INodeId: Into<NodeId<'a>>>(node_id: INodeId) -> AllocationRequestParams<'a> {
        AllocationRequestParams { url_params: AllocationUrlParams::NodeId(node_id.into()) }
    }
}
impl<'a> AllocationRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            AllocationUrlParams::None => Cow::Borrowed("/_cat/allocation"),
            AllocationUrlParams::NodeId(ref node_id) => {
                let mut url = String::with_capacity(17usize + node_id.len());
                url.push_str("/_cat/allocation/");
                url.push_str(node_id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a AllocationRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum FlushUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct FlushRequestParams<'a> {
    pub url_params: FlushUrlParams<'a>,
}
impl<'a> FlushRequestParams<'a> {
    pub fn new() -> FlushRequestParams<'a> {
        FlushRequestParams { url_params: FlushUrlParams::None }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> FlushRequestParams<'a> {
        FlushRequestParams { url_params: FlushUrlParams::Index(index.into()) }
    }
}
impl<'a> FlushRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            FlushUrlParams::None => Cow::Borrowed("/_flush"),
            FlushUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(8usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_flush");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a FlushRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum RefreshUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct RefreshRequestParams<'a> {
    pub url_params: RefreshUrlParams<'a>,
}
impl<'a> RefreshRequestParams<'a> {
    pub fn new() -> RefreshRequestParams<'a> {
        RefreshRequestParams { url_params: RefreshUrlParams::None }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> RefreshRequestParams<'a> {
        RefreshRequestParams { url_params: RefreshUrlParams::Index(index.into()) }
    }
}
impl<'a> RefreshRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            RefreshUrlParams::None => Cow::Borrowed("/_refresh"),
            RefreshUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(10usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_refresh");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a RefreshRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum HelpUrlParams<'a> {
    None,
}
pub struct HelpRequestParams<'a> {
    pub url_params: HelpUrlParams<'a>,
}
impl<'a> HelpRequestParams<'a> {
    pub fn new() -> HelpRequestParams<'a> {
        HelpRequestParams { url_params: HelpUrlParams::None }
    }
}
impl<'a> HelpRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            HelpUrlParams::None => Cow::Borrowed("/_cat"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a HelpRequestParams<'b> {
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
}
impl<'a> SearchShardsRequestParams<'a> {
    pub fn new() -> SearchShardsRequestParams<'a> {
        SearchShardsRequestParams { url_params: SearchShardsUrlParams::None }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> SearchShardsRequestParams<'a> {
        SearchShardsRequestParams { url_params: SearchShardsUrlParams::Index(index.into()) }
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

pub enum HealthUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct HealthRequestParams<'a> {
    pub url_params: HealthUrlParams<'a>,
}
impl<'a> HealthRequestParams<'a> {
    pub fn new() -> HealthRequestParams<'a> {
        HealthRequestParams { url_params: HealthUrlParams::None }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> HealthRequestParams<'a> {
        HealthRequestParams { url_params: HealthUrlParams::Index(index.into()) }
    }
}
impl<'a> HealthRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            HealthUrlParams::None => Cow::Borrowed("/_cluster/health"),
            HealthUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(17usize + index.len());
                url.push_str("/_cluster/health/");
                url.push_str(index.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a HealthRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum ExistsAliasUrlParams<'a> {
    Name(Name<'a>),
    IndexName(Index<'a>, Name<'a>),
    Index(Index<'a>),
}
pub struct ExistsAliasRequestParams<'a> {
    pub url_params: ExistsAliasUrlParams<'a>,
}
impl<'a> ExistsAliasRequestParams<'a> {
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> ExistsAliasRequestParams<'a> {
        ExistsAliasRequestParams { url_params: ExistsAliasUrlParams::Name(name.into()) }
    }
    pub fn index_name<IIndex: Into<Index<'a>>, IName: Into<Name<'a>>>
        (index: IIndex,
         name: IName)
         -> ExistsAliasRequestParams<'a> {
        ExistsAliasRequestParams {
            url_params: ExistsAliasUrlParams::IndexName(index.into(), name.into()),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> ExistsAliasRequestParams<'a> {
        ExistsAliasRequestParams { url_params: ExistsAliasUrlParams::Index(index.into()) }
    }
}
impl<'a> ExistsAliasRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ExistsAliasUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(8usize + name.len());
                url.push_str("/_alias/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
            ExistsAliasUrlParams::IndexName(ref index, ref name) => {
                let mut url = String::with_capacity(9usize + index.len() + name.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_alias/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
            ExistsAliasUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(8usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_alias");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ExistsAliasRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum GetFieldMappingUrlParams<'a> {
    Fields(Fields<'a>),
    IndexFields(Index<'a>, Fields<'a>),
    TypeFields(Type<'a>, Fields<'a>),
    IndexTypeFields(Index<'a>, Type<'a>, Fields<'a>),
}
pub struct GetFieldMappingRequestParams<'a> {
    pub url_params: GetFieldMappingUrlParams<'a>,
}
impl<'a> GetFieldMappingRequestParams<'a> {
    pub fn fields<IFields: Into<Fields<'a>>>(fields: IFields) -> GetFieldMappingRequestParams<'a> {
        GetFieldMappingRequestParams { url_params: GetFieldMappingUrlParams::Fields(fields.into()) }
    }
    pub fn index_fields<IIndex: Into<Index<'a>>, IFields: Into<Fields<'a>>>
        (index: IIndex,
         fields: IFields)
         -> GetFieldMappingRequestParams<'a> {
        GetFieldMappingRequestParams {
            url_params: GetFieldMappingUrlParams::IndexFields(index.into(), fields.into()),
        }
    }
    pub fn ty_fields<IType: Into<Type<'a>>, IFields: Into<Fields<'a>>>
        (ty: IType,
         fields: IFields)
         -> GetFieldMappingRequestParams<'a> {
        GetFieldMappingRequestParams {
            url_params: GetFieldMappingUrlParams::TypeFields(ty.into(), fields.into()),
        }
    }
    pub fn index_ty_fields<IIndex: Into<Index<'a>>,
                           IType: Into<Type<'a>>,
                           IFields: Into<Fields<'a>>>
        (index: IIndex,
         ty: IType,
         fields: IFields)
         -> GetFieldMappingRequestParams<'a> {
        GetFieldMappingRequestParams {
            url_params: GetFieldMappingUrlParams::IndexTypeFields(index.into(),
                                                                  ty.into(),
                                                                  fields.into()),
        }
    }
}
impl<'a> GetFieldMappingRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            GetFieldMappingUrlParams::Fields(ref fields) => {
                let mut url = String::with_capacity(16usize + fields.len());
                url.push_str("/_mapping/field/");
                url.push_str(fields.as_ref());
                Cow::Owned(url)
            }
            GetFieldMappingUrlParams::IndexFields(ref index, ref fields) => {
                let mut url = String::with_capacity(17usize + index.len() + fields.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_mapping/field/");
                url.push_str(fields.as_ref());
                Cow::Owned(url)
            }
            GetFieldMappingUrlParams::TypeFields(ref ty, ref fields) => {
                let mut url = String::with_capacity(17usize + ty.len() + fields.len());
                url.push_str("/_mapping/");
                url.push_str(ty.as_ref());
                url.push_str("/field/");
                url.push_str(fields.as_ref());
                Cow::Owned(url)
            }
            GetFieldMappingUrlParams::IndexTypeFields(ref index, ref ty, ref fields) => {
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
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a GetFieldMappingRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum PutPipelineUrlParams<'a> {
    Id(Id<'a>),
}
pub struct PutPipelineRequestParams<'a> {
    pub url_params: PutPipelineUrlParams<'a>,
    pub body: Body<'a>,
}
impl<'a> PutPipelineRequestParams<'a> {
    pub fn id<IId: Into<Id<'a>>, IBody: Into<Body<'a>>>(id: IId,
                                                        body: IBody)
                                                        -> PutPipelineRequestParams<'a> {
        PutPipelineRequestParams {
            url_params: PutPipelineUrlParams::Id(id.into()),
            body: body.into(),
        }
    }
}
impl<'a> PutPipelineRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            PutPipelineUrlParams::Id(ref id) => {
                let mut url = String::with_capacity(18usize + id.len());
                url.push_str("/_ingest/pipeline/");
                url.push_str(id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a PutPipelineRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum PendingTasksUrlParams<'a> {
    None,
}
pub struct PendingTasksRequestParams<'a> {
    pub url_params: PendingTasksUrlParams<'a>,
}
impl<'a> PendingTasksRequestParams<'a> {
    pub fn new() -> PendingTasksRequestParams<'a> {
        PendingTasksRequestParams { url_params: PendingTasksUrlParams::None }
    }
}
impl<'a> PendingTasksRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            PendingTasksUrlParams::None => Cow::Borrowed("/_cluster/pending_tasks"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a PendingTasksRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum SimulateUrlParams<'a> {
    None,
    Id(Id<'a>),
}
pub struct SimulateRequestParams<'a> {
    pub url_params: SimulateUrlParams<'a>,
    pub body: Body<'a>,
}
impl<'a> SimulateRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> SimulateRequestParams<'a> {
        SimulateRequestParams {
            url_params: SimulateUrlParams::None,
            body: body.into(),
        }
    }
    pub fn id<IId: Into<Id<'a>>, IBody: Into<Body<'a>>>(id: IId,
                                                        body: IBody)
                                                        -> SimulateRequestParams<'a> {
        SimulateRequestParams {
            url_params: SimulateUrlParams::Id(id.into()),
            body: body.into(),
        }
    }
}
impl<'a> SimulateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            SimulateUrlParams::None => Cow::Borrowed("/_ingest/pipeline/_simulate"),
            SimulateUrlParams::Id(ref id) => {
                let mut url = String::with_capacity(28usize + id.len());
                url.push_str("/_ingest/pipeline/");
                url.push_str(id.as_ref());
                url.push_str("/_simulate");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SimulateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum GetAliasUrlParams<'a> {
    None,
    Name(Name<'a>),
    IndexName(Index<'a>, Name<'a>),
    Index(Index<'a>),
}
pub struct GetAliasRequestParams<'a> {
    pub url_params: GetAliasUrlParams<'a>,
}
impl<'a> GetAliasRequestParams<'a> {
    pub fn new() -> GetAliasRequestParams<'a> {
        GetAliasRequestParams { url_params: GetAliasUrlParams::None }
    }
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> GetAliasRequestParams<'a> {
        GetAliasRequestParams { url_params: GetAliasUrlParams::Name(name.into()) }
    }
    pub fn index_name<IIndex: Into<Index<'a>>, IName: Into<Name<'a>>>
        (index: IIndex,
         name: IName)
         -> GetAliasRequestParams<'a> {
        GetAliasRequestParams {
            url_params: GetAliasUrlParams::IndexName(index.into(), name.into()),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> GetAliasRequestParams<'a> {
        GetAliasRequestParams { url_params: GetAliasUrlParams::Index(index.into()) }
    }
}
impl<'a> GetAliasRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            GetAliasUrlParams::None => Cow::Borrowed("/_alias"),
            GetAliasUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(8usize + name.len());
                url.push_str("/_alias/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
            GetAliasUrlParams::IndexName(ref index, ref name) => {
                let mut url = String::with_capacity(9usize + index.len() + name.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_alias/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
            GetAliasUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(8usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_alias");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a GetAliasRequestParams<'b> {
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
}
impl<'a> GetScriptRequestParams<'a> {
    pub fn lang_id<ILang: Into<Lang<'a>>, IId: Into<Id<'a>>>(lang: ILang,
                                                             id: IId)
                                                             -> GetScriptRequestParams<'a> {
        GetScriptRequestParams { url_params: GetScriptUrlParams::LangId(lang.into(), id.into()) }
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

pub enum RecoveryUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct RecoveryRequestParams<'a> {
    pub url_params: RecoveryUrlParams<'a>,
}
impl<'a> RecoveryRequestParams<'a> {
    pub fn new() -> RecoveryRequestParams<'a> {
        RecoveryRequestParams { url_params: RecoveryUrlParams::None }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> RecoveryRequestParams<'a> {
        RecoveryRequestParams { url_params: RecoveryUrlParams::Index(index.into()) }
    }
}
impl<'a> RecoveryRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            RecoveryUrlParams::None => Cow::Borrowed("/_recovery"),
            RecoveryUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(11usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_recovery");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a RecoveryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum DeletePipelineUrlParams<'a> {
    Id(Id<'a>),
}
pub struct DeletePipelineRequestParams<'a> {
    pub url_params: DeletePipelineUrlParams<'a>,
}
impl<'a> DeletePipelineRequestParams<'a> {
    pub fn id<IId: Into<Id<'a>>>(id: IId) -> DeletePipelineRequestParams<'a> {
        DeletePipelineRequestParams { url_params: DeletePipelineUrlParams::Id(id.into()) }
    }
}
impl<'a> DeletePipelineRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            DeletePipelineUrlParams::Id(ref id) => {
                let mut url = String::with_capacity(18usize + id.len());
                url.push_str("/_ingest/pipeline/");
                url.push_str(id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a DeletePipelineRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum CancelUrlParams<'a> {
    None,
    TaskId(TaskId<'a>),
}
pub struct CancelRequestParams<'a> {
    pub url_params: CancelUrlParams<'a>,
}
impl<'a> CancelRequestParams<'a> {
    pub fn new() -> CancelRequestParams<'a> {
        CancelRequestParams { url_params: CancelUrlParams::None }
    }
    pub fn task_id<ITaskId: Into<TaskId<'a>>>(task_id: ITaskId) -> CancelRequestParams<'a> {
        CancelRequestParams { url_params: CancelUrlParams::TaskId(task_id.into()) }
    }
}
impl<'a> CancelRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CancelUrlParams::None => Cow::Borrowed("/_tasks/_cancel"),
            CancelUrlParams::TaskId(ref task_id) => {
                let mut url = String::with_capacity(16usize + task_id.len());
                url.push_str("/_tasks/");
                url.push_str(task_id.as_ref());
                url.push_str("/_cancel");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CancelRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum ClearCacheUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct ClearCacheRequestParams<'a> {
    pub url_params: ClearCacheUrlParams<'a>,
}
impl<'a> ClearCacheRequestParams<'a> {
    pub fn new() -> ClearCacheRequestParams<'a> {
        ClearCacheRequestParams { url_params: ClearCacheUrlParams::None }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> ClearCacheRequestParams<'a> {
        ClearCacheRequestParams { url_params: ClearCacheUrlParams::Index(index.into()) }
    }
}
impl<'a> ClearCacheRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ClearCacheUrlParams::None => Cow::Borrowed("/_cache/clear"),
            ClearCacheUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(14usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_cache/clear");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ClearCacheRequestParams<'b> {
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
}
impl<'a> DeleteRequestParams<'a> {
    pub fn index_ty_id<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IId: Into<Id<'a>>>
        (index: IIndex,
         ty: IType,
         id: IId)
         -> DeleteRequestParams<'a> {
        DeleteRequestParams {
            url_params: DeleteUrlParams::IndexTypeId(index.into(), ty.into(), id.into()),
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

pub enum PutMappingUrlParams<'a> {
    IndexType(Index<'a>, Type<'a>),
    IndexType(Index<'a>, Type<'a>),
    Type(Type<'a>),
    IndexType(Index<'a>, Type<'a>),
    IndexType(Index<'a>, Type<'a>),
    Type(Type<'a>),
}
pub struct PutMappingRequestParams<'a> {
    pub url_params: PutMappingUrlParams<'a>,
    pub body: Body<'a>,
}
impl<'a> PutMappingRequestParams<'a> {
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> PutMappingRequestParams<'a> {
        PutMappingRequestParams {
            url_params: PutMappingUrlParams::IndexType(index.into(), ty.into()),
            body: body.into(),
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> PutMappingRequestParams<'a> {
        PutMappingRequestParams {
            url_params: PutMappingUrlParams::IndexType(index.into(), ty.into()),
            body: body.into(),
        }
    }
    pub fn ty<IType: Into<Type<'a>>, IBody: Into<Body<'a>>>(ty: IType,
                                                            body: IBody)
                                                            -> PutMappingRequestParams<'a> {
        PutMappingRequestParams {
            url_params: PutMappingUrlParams::Type(ty.into()),
            body: body.into(),
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> PutMappingRequestParams<'a> {
        PutMappingRequestParams {
            url_params: PutMappingUrlParams::IndexType(index.into(), ty.into()),
            body: body.into(),
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> PutMappingRequestParams<'a> {
        PutMappingRequestParams {
            url_params: PutMappingUrlParams::IndexType(index.into(), ty.into()),
            body: body.into(),
        }
    }
    pub fn ty<IType: Into<Type<'a>>, IBody: Into<Body<'a>>>(ty: IType,
                                                            body: IBody)
                                                            -> PutMappingRequestParams<'a> {
        PutMappingRequestParams {
            url_params: PutMappingUrlParams::Type(ty.into()),
            body: body.into(),
        }
    }
}
impl<'a> PutMappingRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            PutMappingUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(11usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_mapping");
                Cow::Owned(url)
            }
            PutMappingUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(11usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_mapping/");
                url.push_str(ty.as_ref());
                Cow::Owned(url)
            }
            PutMappingUrlParams::Type(ref ty) => {
                let mut url = String::with_capacity(10usize + ty.len());
                url.push_str("/_mapping/");
                url.push_str(ty.as_ref());
                Cow::Owned(url)
            }
            PutMappingUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(12usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_mappings");
                Cow::Owned(url)
            }
            PutMappingUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(12usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_mappings/");
                url.push_str(ty.as_ref());
                Cow::Owned(url)
            }
            PutMappingUrlParams::Type(ref ty) => {
                let mut url = String::with_capacity(11usize + ty.len());
                url.push_str("/_mappings/");
                url.push_str(ty.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a PutMappingRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum AliasesUrlParams<'a> {
    None,
    Name(Name<'a>),
}
pub struct AliasesRequestParams<'a> {
    pub url_params: AliasesUrlParams<'a>,
}
impl<'a> AliasesRequestParams<'a> {
    pub fn new() -> AliasesRequestParams<'a> {
        AliasesRequestParams { url_params: AliasesUrlParams::None }
    }
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> AliasesRequestParams<'a> {
        AliasesRequestParams { url_params: AliasesUrlParams::Name(name.into()) }
    }
}
impl<'a> AliasesRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            AliasesUrlParams::None => Cow::Borrowed("/_cat/aliases"),
            AliasesUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(14usize + name.len());
                url.push_str("/_cat/aliases/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a AliasesRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum StatsUrlParams<'a> {
    None,
    NodeId(NodeId<'a>),
}
pub struct StatsRequestParams<'a> {
    pub url_params: StatsUrlParams<'a>,
}
impl<'a> StatsRequestParams<'a> {
    pub fn new() -> StatsRequestParams<'a> {
        StatsRequestParams { url_params: StatsUrlParams::None }
    }
    pub fn node_id<INodeId: Into<NodeId<'a>>>(node_id: INodeId) -> StatsRequestParams<'a> {
        StatsRequestParams { url_params: StatsUrlParams::NodeId(node_id.into()) }
    }
}
impl<'a> StatsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            StatsUrlParams::None => Cow::Borrowed("/_cluster/stats"),
            StatsUrlParams::NodeId(ref node_id) => {
                let mut url = String::with_capacity(22usize + node_id.len());
                url.push_str("/_cluster/stats/nodes/");
                url.push_str(node_id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a StatsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum ValidateQueryUrlParams<'a> {
    None,
    Index(Index<'a>),
    IndexType(Index<'a>, Type<'a>),
}
pub struct ValidateQueryRequestParams<'a> {
    pub url_params: ValidateQueryUrlParams<'a>,
    pub body: Body<'a>,
}
impl<'a> ValidateQueryRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> ValidateQueryRequestParams<'a> {
        ValidateQueryRequestParams {
            url_params: ValidateQueryUrlParams::None,
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> ValidateQueryRequestParams<'a> {
        ValidateQueryRequestParams {
            url_params: ValidateQueryUrlParams::Index(index.into()),
            body: body.into(),
        }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         ty: IType,
         body: IBody)
         -> ValidateQueryRequestParams<'a> {
        ValidateQueryRequestParams {
            url_params: ValidateQueryUrlParams::IndexType(index.into(), ty.into()),
            body: body.into(),
        }
    }
}
impl<'a> ValidateQueryRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ValidateQueryUrlParams::None => Cow::Borrowed("/_validate/query"),
            ValidateQueryUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(17usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_validate/query");
                Cow::Owned(url)
            }
            ValidateQueryUrlParams::IndexType(ref index, ref ty) => {
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
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ValidateQueryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum PendingTasksUrlParams<'a> {
    None,
}
pub struct PendingTasksRequestParams<'a> {
    pub url_params: PendingTasksUrlParams<'a>,
}
impl<'a> PendingTasksRequestParams<'a> {
    pub fn new() -> PendingTasksRequestParams<'a> {
        PendingTasksRequestParams { url_params: PendingTasksUrlParams::None }
    }
}
impl<'a> PendingTasksRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            PendingTasksUrlParams::None => Cow::Borrowed("/_cat/pending_tasks"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a PendingTasksRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum ClearScrollUrlParams<'a> {
    ScrollId(ScrollId<'a>),
    None,
}
pub struct ClearScrollRequestParams<'a> {
    pub url_params: ClearScrollUrlParams<'a>,
    pub body: Body<'a>,
}
impl<'a> ClearScrollRequestParams<'a> {
    pub fn scroll_id<IScrollId: Into<ScrollId<'a>>, IBody: Into<Body<'a>>>
        (scroll_id: IScrollId,
         body: IBody)
         -> ClearScrollRequestParams<'a> {
        ClearScrollRequestParams {
            url_params: ClearScrollUrlParams::ScrollId(scroll_id.into()),
            body: body.into(),
        }
    }
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> ClearScrollRequestParams<'a> {
        ClearScrollRequestParams {
            url_params: ClearScrollUrlParams::None,
            body: body.into(),
        }
    }
}
impl<'a> ClearScrollRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ClearScrollUrlParams::ScrollId(ref scroll_id) => {
                let mut url = String::with_capacity(16usize + scroll_id.len());
                url.push_str("/_search/scroll/");
                url.push_str(scroll_id.as_ref());
                Cow::Owned(url)
            }
            ClearScrollUrlParams::None => Cow::Borrowed("/_search/scroll"),
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

pub enum ShardsUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct ShardsRequestParams<'a> {
    pub url_params: ShardsUrlParams<'a>,
}
impl<'a> ShardsRequestParams<'a> {
    pub fn new() -> ShardsRequestParams<'a> {
        ShardsRequestParams { url_params: ShardsUrlParams::None }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> ShardsRequestParams<'a> {
        ShardsRequestParams { url_params: ShardsUrlParams::Index(index.into()) }
    }
}
impl<'a> ShardsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ShardsUrlParams::None => Cow::Borrowed("/_cat/shards"),
            ShardsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(13usize + index.len());
                url.push_str("/_cat/shards/");
                url.push_str(index.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ShardsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum ShardStoresUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct ShardStoresRequestParams<'a> {
    pub url_params: ShardStoresUrlParams<'a>,
}
impl<'a> ShardStoresRequestParams<'a> {
    pub fn new() -> ShardStoresRequestParams<'a> {
        ShardStoresRequestParams { url_params: ShardStoresUrlParams::None }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> ShardStoresRequestParams<'a> {
        ShardStoresRequestParams { url_params: ShardStoresUrlParams::Index(index.into()) }
    }
}
impl<'a> ShardStoresRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ShardStoresUrlParams::None => Cow::Borrowed("/_shard_stores"),
            ShardStoresUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(15usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_shard_stores");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ShardStoresRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum UpdateAliasesUrlParams<'a> {
    None,
}
pub struct UpdateAliasesRequestParams<'a> {
    pub url_params: UpdateAliasesUrlParams<'a>,
    pub body: Body<'a>,
}
impl<'a> UpdateAliasesRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> UpdateAliasesRequestParams<'a> {
        UpdateAliasesRequestParams {
            url_params: UpdateAliasesUrlParams::None,
            body: body.into(),
        }
    }
}
impl<'a> UpdateAliasesRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            UpdateAliasesUrlParams::None => Cow::Borrowed("/_aliases"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a UpdateAliasesRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum SegmentsUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct SegmentsRequestParams<'a> {
    pub url_params: SegmentsUrlParams<'a>,
}
impl<'a> SegmentsRequestParams<'a> {
    pub fn new() -> SegmentsRequestParams<'a> {
        SegmentsRequestParams { url_params: SegmentsUrlParams::None }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> SegmentsRequestParams<'a> {
        SegmentsRequestParams { url_params: SegmentsUrlParams::Index(index.into()) }
    }
}
impl<'a> SegmentsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            SegmentsUrlParams::None => Cow::Borrowed("/_cat/segments"),
            SegmentsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(15usize + index.len());
                url.push_str("/_cat/segments/");
                url.push_str(index.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SegmentsRequestParams<'b> {
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
}
impl<'a> MpercolateRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> MpercolateRequestParams<'a> {
        MpercolateRequestParams {
            url_params: MpercolateUrlParams::None,
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> MpercolateRequestParams<'a> {
        MpercolateRequestParams {
            url_params: MpercolateUrlParams::Index(index.into()),
            body: body.into(),
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

pub enum OpenUrlParams<'a> {
    Index(Index<'a>),
}
pub struct OpenRequestParams<'a> {
    pub url_params: OpenUrlParams<'a>,
}
impl<'a> OpenRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> OpenRequestParams<'a> {
        OpenRequestParams { url_params: OpenUrlParams::Index(index.into()) }
    }
}
impl<'a> OpenRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            OpenUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(7usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_open");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a OpenRequestParams<'b> {
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
}
impl<'a> GetRequestParams<'a> {
    pub fn index_ty_id<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IId: Into<Id<'a>>>
        (index: IIndex,
         ty: IType,
         id: IId)
         -> GetRequestParams<'a> {
        GetRequestParams {
            url_params: GetUrlParams::IndexTypeId(index.into(), ty.into(), id.into()),
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
}
impl<'a> UpdateByQueryRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> UpdateByQueryRequestParams<'a> {
        UpdateByQueryRequestParams {
            url_params: UpdateByQueryUrlParams::Index(index.into()),
            body: body.into(),
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
}
impl<'a> MtermvectorsRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> MtermvectorsRequestParams<'a> {
        MtermvectorsRequestParams {
            url_params: MtermvectorsUrlParams::None,
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> MtermvectorsRequestParams<'a> {
        MtermvectorsRequestParams {
            url_params: MtermvectorsUrlParams::Index(index.into()),
            body: body.into(),
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

pub enum RecoveryUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct RecoveryRequestParams<'a> {
    pub url_params: RecoveryUrlParams<'a>,
}
impl<'a> RecoveryRequestParams<'a> {
    pub fn new() -> RecoveryRequestParams<'a> {
        RecoveryRequestParams { url_params: RecoveryUrlParams::None }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> RecoveryRequestParams<'a> {
        RecoveryRequestParams { url_params: RecoveryUrlParams::Index(index.into()) }
    }
}
impl<'a> RecoveryRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            RecoveryUrlParams::None => Cow::Borrowed("/_cat/recovery"),
            RecoveryUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(15usize + index.len());
                url.push_str("/_cat/recovery/");
                url.push_str(index.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a RecoveryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum RestoreUrlParams<'a> {
    RepositorySnapshot(Repository<'a>, Snapshot<'a>),
}
pub struct RestoreRequestParams<'a> {
    pub url_params: RestoreUrlParams<'a>,
    pub body: Body<'a>,
}
impl<'a> RestoreRequestParams<'a> {
    pub fn repository_snapshot<IRepository: Into<Repository<'a>>,
                               ISnapshot: Into<Snapshot<'a>>,
                               IBody: Into<Body<'a>>>
        (repository: IRepository,
         snapshot: ISnapshot,
         body: IBody)
         -> RestoreRequestParams<'a> {
        RestoreRequestParams {
            url_params: RestoreUrlParams::RepositorySnapshot(repository.into(), snapshot.into()),
            body: body.into(),
        }
    }
}
impl<'a> RestoreRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            RestoreUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
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
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a RestoreRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum ReindexUrlParams<'a> {
    None,
}
pub struct ReindexRequestParams<'a> {
    pub url_params: ReindexUrlParams<'a>,
    pub body: Body<'a>,
}
impl<'a> ReindexRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> ReindexRequestParams<'a> {
        ReindexRequestParams {
            url_params: ReindexUrlParams::None,
            body: body.into(),
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

pub enum HealthUrlParams<'a> {
    None,
}
pub struct HealthRequestParams<'a> {
    pub url_params: HealthUrlParams<'a>,
}
impl<'a> HealthRequestParams<'a> {
    pub fn new() -> HealthRequestParams<'a> {
        HealthRequestParams { url_params: HealthUrlParams::None }
    }
}
impl<'a> HealthRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            HealthUrlParams::None => Cow::Borrowed("/_cat/health"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a HealthRequestParams<'b> {
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
}
pub struct CountRequestParams<'a> {
    pub url_params: CountUrlParams<'a>,
}
impl<'a> CountRequestParams<'a> {
    pub fn new() -> CountRequestParams<'a> {
        CountRequestParams { url_params: CountUrlParams::None }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> CountRequestParams<'a> {
        CountRequestParams { url_params: CountUrlParams::Index(index.into()) }
    }
}
impl<'a> CountRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CountUrlParams::None => Cow::Borrowed("/_cat/count"),
            CountUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(12usize + index.len());
                url.push_str("/_cat/count/");
                url.push_str(index.as_ref());
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
            body: None,
        }
    }
}

pub enum SnapshotsUrlParams<'a> {
    None,
    Repository(Repository<'a>),
}
pub struct SnapshotsRequestParams<'a> {
    pub url_params: SnapshotsUrlParams<'a>,
}
impl<'a> SnapshotsRequestParams<'a> {
    pub fn new() -> SnapshotsRequestParams<'a> {
        SnapshotsRequestParams { url_params: SnapshotsUrlParams::None }
    }
    pub fn repository<IRepository: Into<Repository<'a>>>(repository: IRepository)
                                                         -> SnapshotsRequestParams<'a> {
        SnapshotsRequestParams { url_params: SnapshotsUrlParams::Repository(repository.into()) }
    }
}
impl<'a> SnapshotsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            SnapshotsUrlParams::None => Cow::Borrowed("/_cat/snapshots"),
            SnapshotsUrlParams::Repository(ref repository) => {
                let mut url = String::with_capacity(16usize + repository.len());
                url.push_str("/_cat/snapshots/");
                url.push_str(repository.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SnapshotsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum GetMappingUrlParams<'a> {
    None,
    Index(Index<'a>),
    Type(Type<'a>),
    IndexType(Index<'a>, Type<'a>),
}
pub struct GetMappingRequestParams<'a> {
    pub url_params: GetMappingUrlParams<'a>,
}
impl<'a> GetMappingRequestParams<'a> {
    pub fn new() -> GetMappingRequestParams<'a> {
        GetMappingRequestParams { url_params: GetMappingUrlParams::None }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> GetMappingRequestParams<'a> {
        GetMappingRequestParams { url_params: GetMappingUrlParams::Index(index.into()) }
    }
    pub fn ty<IType: Into<Type<'a>>>(ty: IType) -> GetMappingRequestParams<'a> {
        GetMappingRequestParams { url_params: GetMappingUrlParams::Type(ty.into()) }
    }
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>>
        (index: IIndex,
         ty: IType)
         -> GetMappingRequestParams<'a> {
        GetMappingRequestParams {
            url_params: GetMappingUrlParams::IndexType(index.into(), ty.into()),
        }
    }
}
impl<'a> GetMappingRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            GetMappingUrlParams::None => Cow::Borrowed("/_mapping"),
            GetMappingUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(10usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_mapping");
                Cow::Owned(url)
            }
            GetMappingUrlParams::Type(ref ty) => {
                let mut url = String::with_capacity(10usize + ty.len());
                url.push_str("/_mapping/");
                url.push_str(ty.as_ref());
                Cow::Owned(url)
            }
            GetMappingUrlParams::IndexType(ref index, ref ty) => {
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
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a GetMappingRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum GetUrlParams<'a> {
    RepositorySnapshot(Repository<'a>, Snapshot<'a>),
}
pub struct GetRequestParams<'a> {
    pub url_params: GetUrlParams<'a>,
}
impl<'a> GetRequestParams<'a> {
    pub fn repository_snapshot<IRepository: Into<Repository<'a>>, ISnapshot: Into<Snapshot<'a>>>
        (repository: IRepository,
         snapshot: ISnapshot)
         -> GetRequestParams<'a> {
        GetRequestParams {
            url_params: GetUrlParams::RepositorySnapshot(repository.into(), snapshot.into()),
        }
    }
}
impl<'a> GetRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            GetUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
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
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a GetRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum NodesUrlParams<'a> {
    None,
}
pub struct NodesRequestParams<'a> {
    pub url_params: NodesUrlParams<'a>,
}
impl<'a> NodesRequestParams<'a> {
    pub fn new() -> NodesRequestParams<'a> {
        NodesRequestParams { url_params: NodesUrlParams::None }
    }
}
impl<'a> NodesRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            NodesUrlParams::None => Cow::Borrowed("/_cat/nodes"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a NodesRequestParams<'b> {
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
}
impl<'a> ExistsRequestParams<'a> {
    pub fn index_ty_id<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IId: Into<Id<'a>>>
        (index: IIndex,
         ty: IType,
         id: IId)
         -> ExistsRequestParams<'a> {
        ExistsRequestParams {
            url_params: ExistsUrlParams::IndexTypeId(index.into(), ty.into(), id.into()),
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

pub enum RerouteUrlParams<'a> {
    None,
}
pub struct RerouteRequestParams<'a> {
    pub url_params: RerouteUrlParams<'a>,
    pub body: Body<'a>,
}
impl<'a> RerouteRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> RerouteRequestParams<'a> {
        RerouteRequestParams {
            url_params: RerouteUrlParams::None,
            body: body.into(),
        }
    }
}
impl<'a> RerouteRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            RerouteUrlParams::None => Cow::Borrowed("/_cluster/reroute"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a RerouteRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum HotThreadsUrlParams<'a> {
    None,
    None,
    NodeId(NodeId<'a>),
    NodeId(NodeId<'a>),
    None,
    None,
    NodeId(NodeId<'a>),
    NodeId(NodeId<'a>),
}
pub struct HotThreadsRequestParams<'a> {
    pub url_params: HotThreadsUrlParams<'a>,
}
impl<'a> HotThreadsRequestParams<'a> {
    pub fn new() -> HotThreadsRequestParams<'a> {
        HotThreadsRequestParams { url_params: HotThreadsUrlParams::None }
    }
    pub fn new() -> HotThreadsRequestParams<'a> {
        HotThreadsRequestParams { url_params: HotThreadsUrlParams::None }
    }
    pub fn node_id<INodeId: Into<NodeId<'a>>>(node_id: INodeId) -> HotThreadsRequestParams<'a> {
        HotThreadsRequestParams { url_params: HotThreadsUrlParams::NodeId(node_id.into()) }
    }
    pub fn node_id<INodeId: Into<NodeId<'a>>>(node_id: INodeId) -> HotThreadsRequestParams<'a> {
        HotThreadsRequestParams { url_params: HotThreadsUrlParams::NodeId(node_id.into()) }
    }
    pub fn new() -> HotThreadsRequestParams<'a> {
        HotThreadsRequestParams { url_params: HotThreadsUrlParams::None }
    }
    pub fn new() -> HotThreadsRequestParams<'a> {
        HotThreadsRequestParams { url_params: HotThreadsUrlParams::None }
    }
    pub fn node_id<INodeId: Into<NodeId<'a>>>(node_id: INodeId) -> HotThreadsRequestParams<'a> {
        HotThreadsRequestParams { url_params: HotThreadsUrlParams::NodeId(node_id.into()) }
    }
    pub fn node_id<INodeId: Into<NodeId<'a>>>(node_id: INodeId) -> HotThreadsRequestParams<'a> {
        HotThreadsRequestParams { url_params: HotThreadsUrlParams::NodeId(node_id.into()) }
    }
}
impl<'a> HotThreadsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            HotThreadsUrlParams::None => Cow::Borrowed("/_cluster/nodes/hotthreads"),
            HotThreadsUrlParams::None => Cow::Borrowed("/_cluster/nodes/hot_threads"),
            HotThreadsUrlParams::NodeId(ref node_id) => {
                let mut url = String::with_capacity(27usize + node_id.len());
                url.push_str("/_cluster/nodes/");
                url.push_str(node_id.as_ref());
                url.push_str("/hotthreads");
                Cow::Owned(url)
            }
            HotThreadsUrlParams::NodeId(ref node_id) => {
                let mut url = String::with_capacity(28usize + node_id.len());
                url.push_str("/_cluster/nodes/");
                url.push_str(node_id.as_ref());
                url.push_str("/hot_threads");
                Cow::Owned(url)
            }
            HotThreadsUrlParams::None => Cow::Borrowed("/_nodes/hotthreads"),
            HotThreadsUrlParams::None => Cow::Borrowed("/_nodes/hot_threads"),
            HotThreadsUrlParams::NodeId(ref node_id) => {
                let mut url = String::with_capacity(19usize + node_id.len());
                url.push_str("/_nodes/");
                url.push_str(node_id.as_ref());
                url.push_str("/hotthreads");
                Cow::Owned(url)
            }
            HotThreadsUrlParams::NodeId(ref node_id) => {
                let mut url = String::with_capacity(20usize + node_id.len());
                url.push_str("/_nodes/");
                url.push_str(node_id.as_ref());
                url.push_str("/hot_threads");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a HotThreadsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum StatsUrlParams<'a> {
    None,
    NodeId(NodeId<'a>),
    Metric(Metric<'a>),
    NodeIdMetric(NodeId<'a>, Metric<'a>),
    MetricIndexMetric(Metric<'a>, IndexMetric<'a>),
    NodeIdMetricIndexMetric(NodeId<'a>, Metric<'a>, IndexMetric<'a>),
}
pub struct StatsRequestParams<'a> {
    pub url_params: StatsUrlParams<'a>,
}
impl<'a> StatsRequestParams<'a> {
    pub fn new() -> StatsRequestParams<'a> {
        StatsRequestParams { url_params: StatsUrlParams::None }
    }
    pub fn node_id<INodeId: Into<NodeId<'a>>>(node_id: INodeId) -> StatsRequestParams<'a> {
        StatsRequestParams { url_params: StatsUrlParams::NodeId(node_id.into()) }
    }
    pub fn metric<IMetric: Into<Metric<'a>>>(metric: IMetric) -> StatsRequestParams<'a> {
        StatsRequestParams { url_params: StatsUrlParams::Metric(metric.into()) }
    }
    pub fn node_id_metric<INodeId: Into<NodeId<'a>>, IMetric: Into<Metric<'a>>>
        (node_id: INodeId,
         metric: IMetric)
         -> StatsRequestParams<'a> {
        StatsRequestParams {
            url_params: StatsUrlParams::NodeIdMetric(node_id.into(), metric.into()),
        }
    }
    pub fn metric_index_metric<IMetric: Into<Metric<'a>>, IIndexMetric: Into<IndexMetric<'a>>>
        (metric: IMetric,
         index_metric: IIndexMetric)
         -> StatsRequestParams<'a> {
        StatsRequestParams {
            url_params: StatsUrlParams::MetricIndexMetric(metric.into(), index_metric.into()),
        }
    }
    pub fn node_id_metric_index_metric<INodeId: Into<NodeId<'a>>,
                                       IMetric: Into<Metric<'a>>,
                                       IIndexMetric: Into<IndexMetric<'a>>>
        (node_id: INodeId,
         metric: IMetric,
         index_metric: IIndexMetric)
         -> StatsRequestParams<'a> {
        StatsRequestParams {
            url_params: StatsUrlParams::NodeIdMetricIndexMetric(node_id.into(),
                                                                metric.into(),
                                                                index_metric.into()),
        }
    }
}
impl<'a> StatsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            StatsUrlParams::None => Cow::Borrowed("/_nodes/stats"),
            StatsUrlParams::NodeId(ref node_id) => {
                let mut url = String::with_capacity(14usize + node_id.len());
                url.push_str("/_nodes/");
                url.push_str(node_id.as_ref());
                url.push_str("/stats");
                Cow::Owned(url)
            }
            StatsUrlParams::Metric(ref metric) => {
                let mut url = String::with_capacity(14usize + metric.len());
                url.push_str("/_nodes/stats/");
                url.push_str(metric.as_ref());
                Cow::Owned(url)
            }
            StatsUrlParams::NodeIdMetric(ref node_id, ref metric) => {
                let mut url = String::with_capacity(15usize + node_id.len() + metric.len());
                url.push_str("/_nodes/");
                url.push_str(node_id.as_ref());
                url.push_str("/stats/");
                url.push_str(metric.as_ref());
                Cow::Owned(url)
            }
            StatsUrlParams::MetricIndexMetric(ref metric, ref index_metric) => {
                let mut url = String::with_capacity(15usize + metric.len() + index_metric.len());
                url.push_str("/_nodes/stats/");
                url.push_str(metric.as_ref());
                url.push_str("/");
                url.push_str(index_metric.as_ref());
                Cow::Owned(url)
            }
            StatsUrlParams::NodeIdMetricIndexMetric(ref node_id, ref metric, ref index_metric) => {
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
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a StatsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum GetPipelineUrlParams<'a> {
    None,
    Id(Id<'a>),
}
pub struct GetPipelineRequestParams<'a> {
    pub url_params: GetPipelineUrlParams<'a>,
}
impl<'a> GetPipelineRequestParams<'a> {
    pub fn new() -> GetPipelineRequestParams<'a> {
        GetPipelineRequestParams { url_params: GetPipelineUrlParams::None }
    }
    pub fn id<IId: Into<Id<'a>>>(id: IId) -> GetPipelineRequestParams<'a> {
        GetPipelineRequestParams { url_params: GetPipelineUrlParams::Id(id.into()) }
    }
}
impl<'a> GetPipelineRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            GetPipelineUrlParams::None => Cow::Borrowed("/_ingest/pipeline"),
            GetPipelineUrlParams::Id(ref id) => {
                let mut url = String::with_capacity(18usize + id.len());
                url.push_str("/_ingest/pipeline/");
                url.push_str(id.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a GetPipelineRequestParams<'b> {
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
}
impl<'a> PutTemplateRequestParams<'a> {
    pub fn id<IId: Into<Id<'a>>, IBody: Into<Body<'a>>>(id: IId,
                                                        body: IBody)
                                                        -> PutTemplateRequestParams<'a> {
        PutTemplateRequestParams {
            url_params: PutTemplateUrlParams::Id(id.into()),
            body: body.into(),
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
}
impl<'a> GetSourceRequestParams<'a> {
    pub fn index_ty_id<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>, IId: Into<Id<'a>>>
        (index: IIndex,
         ty: IType,
         id: IId)
         -> GetSourceRequestParams<'a> {
        GetSourceRequestParams {
            url_params: GetSourceUrlParams::IndexTypeId(index.into(), ty.into(), id.into()),
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

pub enum CreateUrlParams<'a> {
    RepositorySnapshot(Repository<'a>, Snapshot<'a>),
}
pub struct CreateRequestParams<'a> {
    pub url_params: CreateUrlParams<'a>,
    pub body: Body<'a>,
}
impl<'a> CreateRequestParams<'a> {
    pub fn repository_snapshot<IRepository: Into<Repository<'a>>,
                               ISnapshot: Into<Snapshot<'a>>,
                               IBody: Into<Body<'a>>>
        (repository: IRepository,
         snapshot: ISnapshot,
         body: IBody)
         -> CreateRequestParams<'a> {
        CreateRequestParams {
            url_params: CreateUrlParams::RepositorySnapshot(repository.into(), snapshot.into()),
            body: body.into(),
        }
    }
}
impl<'a> CreateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CreateUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
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
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CreateRequestParams<'b> {
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
}
impl<'a> ScrollRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> ScrollRequestParams<'a> {
        ScrollRequestParams {
            url_params: ScrollUrlParams::None,
            body: body.into(),
        }
    }
    pub fn scroll_id<IScrollId: Into<ScrollId<'a>>, IBody: Into<Body<'a>>>
        (scroll_id: IScrollId,
         body: IBody)
         -> ScrollRequestParams<'a> {
        ScrollRequestParams {
            url_params: ScrollUrlParams::ScrollId(scroll_id.into()),
            body: body.into(),
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

pub enum StatusUrlParams<'a> {
    None,
    Repository(Repository<'a>),
    RepositorySnapshot(Repository<'a>, Snapshot<'a>),
}
pub struct StatusRequestParams<'a> {
    pub url_params: StatusUrlParams<'a>,
}
impl<'a> StatusRequestParams<'a> {
    pub fn new() -> StatusRequestParams<'a> {
        StatusRequestParams { url_params: StatusUrlParams::None }
    }
    pub fn repository<IRepository: Into<Repository<'a>>>(repository: IRepository)
                                                         -> StatusRequestParams<'a> {
        StatusRequestParams { url_params: StatusUrlParams::Repository(repository.into()) }
    }
    pub fn repository_snapshot<IRepository: Into<Repository<'a>>, ISnapshot: Into<Snapshot<'a>>>
        (repository: IRepository,
         snapshot: ISnapshot)
         -> StatusRequestParams<'a> {
        StatusRequestParams {
            url_params: StatusUrlParams::RepositorySnapshot(repository.into(), snapshot.into()),
        }
    }
}
impl<'a> StatusRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            StatusUrlParams::None => Cow::Borrowed("/_snapshot/_status"),
            StatusUrlParams::Repository(ref repository) => {
                let mut url = String::with_capacity(19usize + repository.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                url.push_str("/_status");
                Cow::Owned(url)
            }
            StatusUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
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
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a StatusRequestParams<'b> {
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
}
impl<'a> MgetRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> MgetRequestParams<'a> {
        MgetRequestParams {
            url_params: MgetUrlParams::None,
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> MgetRequestParams<'a> {
        MgetRequestParams {
            url_params: MgetUrlParams::Index(index.into()),
            body: body.into(),
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

pub enum ExistsTemplateUrlParams<'a> {
    Name(Name<'a>),
}
pub struct ExistsTemplateRequestParams<'a> {
    pub url_params: ExistsTemplateUrlParams<'a>,
}
impl<'a> ExistsTemplateRequestParams<'a> {
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> ExistsTemplateRequestParams<'a> {
        ExistsTemplateRequestParams { url_params: ExistsTemplateUrlParams::Name(name.into()) }
    }
}
impl<'a> ExistsTemplateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ExistsTemplateUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(11usize + name.len());
                url.push_str("/_template/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ExistsTemplateRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum GetUpgradeUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct GetUpgradeRequestParams<'a> {
    pub url_params: GetUpgradeUrlParams<'a>,
}
impl<'a> GetUpgradeRequestParams<'a> {
    pub fn new() -> GetUpgradeRequestParams<'a> {
        GetUpgradeRequestParams { url_params: GetUpgradeUrlParams::None }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> GetUpgradeRequestParams<'a> {
        GetUpgradeRequestParams { url_params: GetUpgradeUrlParams::Index(index.into()) }
    }
}
impl<'a> GetUpgradeRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            GetUpgradeUrlParams::None => Cow::Borrowed("/_upgrade"),
            GetUpgradeUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(10usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_upgrade");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a GetUpgradeRequestParams<'b> {
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
}
impl<'a> GetTemplateRequestParams<'a> {
    pub fn id<IId: Into<Id<'a>>>(id: IId) -> GetTemplateRequestParams<'a> {
        GetTemplateRequestParams { url_params: GetTemplateUrlParams::Id(id.into()) }
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

pub enum DeleteTemplateUrlParams<'a> {
    Name(Name<'a>),
}
pub struct DeleteTemplateRequestParams<'a> {
    pub url_params: DeleteTemplateUrlParams<'a>,
}
impl<'a> DeleteTemplateRequestParams<'a> {
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> DeleteTemplateRequestParams<'a> {
        DeleteTemplateRequestParams { url_params: DeleteTemplateUrlParams::Name(name.into()) }
    }
}
impl<'a> DeleteTemplateRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            DeleteTemplateUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(11usize + name.len());
                url.push_str("/_template/");
                url.push_str(name.as_ref());
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

pub enum IndexUrlParams<'a> {
    IndexType(Index<'a>, Type<'a>),
    IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
}
pub struct IndexRequestParams<'a> {
    pub url_params: IndexUrlParams<'a>,
    pub body: Body<'a>,
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

pub enum PutSettingsUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct PutSettingsRequestParams<'a> {
    pub url_params: PutSettingsUrlParams<'a>,
    pub body: Body<'a>,
}
impl<'a> PutSettingsRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> PutSettingsRequestParams<'a> {
        PutSettingsRequestParams {
            url_params: PutSettingsUrlParams::None,
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         body: IBody)
         -> PutSettingsRequestParams<'a> {
        PutSettingsRequestParams {
            url_params: PutSettingsUrlParams::Index(index.into()),
            body: body.into(),
        }
    }
}
impl<'a> PutSettingsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            PutSettingsUrlParams::None => Cow::Borrowed("/_settings"),
            PutSettingsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(11usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_settings");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a PutSettingsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum TemplatesUrlParams<'a> {
    None,
    Name(Name<'a>),
}
pub struct TemplatesRequestParams<'a> {
    pub url_params: TemplatesUrlParams<'a>,
}
impl<'a> TemplatesRequestParams<'a> {
    pub fn new() -> TemplatesRequestParams<'a> {
        TemplatesRequestParams { url_params: TemplatesUrlParams::None }
    }
    pub fn name<IName: Into<Name<'a>>>(name: IName) -> TemplatesRequestParams<'a> {
        TemplatesRequestParams { url_params: TemplatesUrlParams::Name(name.into()) }
    }
}
impl<'a> TemplatesRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            TemplatesUrlParams::None => Cow::Borrowed("/_cat/templates"),
            TemplatesUrlParams::Name(ref name) => {
                let mut url = String::with_capacity(16usize + name.len());
                url.push_str("/_cat/templates/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a TemplatesRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum IndicesUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct IndicesRequestParams<'a> {
    pub url_params: IndicesUrlParams<'a>,
}
impl<'a> IndicesRequestParams<'a> {
    pub fn new() -> IndicesRequestParams<'a> {
        IndicesRequestParams { url_params: IndicesUrlParams::None }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> IndicesRequestParams<'a> {
        IndicesRequestParams { url_params: IndicesUrlParams::Index(index.into()) }
    }
}
impl<'a> IndicesRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            IndicesUrlParams::None => Cow::Borrowed("/_cat/indices"),
            IndicesUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(14usize + index.len());
                url.push_str("/_cat/indices/");
                url.push_str(index.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a IndicesRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum PutSettingsUrlParams<'a> {
    None,
}
pub struct PutSettingsRequestParams<'a> {
    pub url_params: PutSettingsUrlParams<'a>,
    pub body: Body<'a>,
}
impl<'a> PutSettingsRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> PutSettingsRequestParams<'a> {
        PutSettingsRequestParams {
            url_params: PutSettingsUrlParams::None,
            body: body.into(),
        }
    }
}
impl<'a> PutSettingsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            PutSettingsUrlParams::None => Cow::Borrowed("/_cluster/settings"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a PutSettingsRequestParams<'b> {
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

pub enum PutAliasUrlParams<'a> {
    IndexName(Index<'a>, Name<'a>),
    IndexName(Index<'a>, Name<'a>),
}
pub struct PutAliasRequestParams<'a> {
    pub url_params: PutAliasUrlParams<'a>,
    pub body: Body<'a>,
}
impl<'a> PutAliasRequestParams<'a> {
    pub fn index_name<IIndex: Into<Index<'a>>, IName: Into<Name<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         name: IName,
         body: IBody)
         -> PutAliasRequestParams<'a> {
        PutAliasRequestParams {
            url_params: PutAliasUrlParams::IndexName(index.into(), name.into()),
            body: body.into(),
        }
    }
    pub fn index_name<IIndex: Into<Index<'a>>, IName: Into<Name<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         name: IName,
         body: IBody)
         -> PutAliasRequestParams<'a> {
        PutAliasRequestParams {
            url_params: PutAliasUrlParams::IndexName(index.into(), name.into()),
            body: body.into(),
        }
    }
}
impl<'a> PutAliasRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            PutAliasUrlParams::IndexName(ref index, ref name) => {
                let mut url = String::with_capacity(9usize + index.len() + name.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_alias/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
            PutAliasUrlParams::IndexName(ref index, ref name) => {
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
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a PutAliasRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum PluginsUrlParams<'a> {
    None,
}
pub struct PluginsRequestParams<'a> {
    pub url_params: PluginsUrlParams<'a>,
}
impl<'a> PluginsRequestParams<'a> {
    pub fn new() -> PluginsRequestParams<'a> {
        PluginsRequestParams { url_params: PluginsUrlParams::None }
    }
}
impl<'a> PluginsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            PluginsUrlParams::None => Cow::Borrowed("/_cat/plugins"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a PluginsRequestParams<'b> {
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

pub enum UpgradeUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct UpgradeRequestParams<'a> {
    pub url_params: UpgradeUrlParams<'a>,
}
impl<'a> UpgradeRequestParams<'a> {
    pub fn new() -> UpgradeRequestParams<'a> {
        UpgradeRequestParams { url_params: UpgradeUrlParams::None }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> UpgradeRequestParams<'a> {
        UpgradeRequestParams { url_params: UpgradeUrlParams::Index(index.into()) }
    }
}
impl<'a> UpgradeRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            UpgradeUrlParams::None => Cow::Borrowed("/_upgrade"),
            UpgradeUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(10usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_upgrade");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a UpgradeRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum DeleteAliasUrlParams<'a> {
    IndexName(Index<'a>, Name<'a>),
    IndexName(Index<'a>, Name<'a>),
}
pub struct DeleteAliasRequestParams<'a> {
    pub url_params: DeleteAliasUrlParams<'a>,
}
impl<'a> DeleteAliasRequestParams<'a> {
    pub fn index_name<IIndex: Into<Index<'a>>, IName: Into<Name<'a>>>
        (index: IIndex,
         name: IName)
         -> DeleteAliasRequestParams<'a> {
        DeleteAliasRequestParams {
            url_params: DeleteAliasUrlParams::IndexName(index.into(), name.into()),
        }
    }
    pub fn index_name<IIndex: Into<Index<'a>>, IName: Into<Name<'a>>>
        (index: IIndex,
         name: IName)
         -> DeleteAliasRequestParams<'a> {
        DeleteAliasRequestParams {
            url_params: DeleteAliasUrlParams::IndexName(index.into(), name.into()),
        }
    }
}
impl<'a> DeleteAliasRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            DeleteAliasUrlParams::IndexName(ref index, ref name) => {
                let mut url = String::with_capacity(9usize + index.len() + name.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_alias/");
                url.push_str(name.as_ref());
                Cow::Owned(url)
            }
            DeleteAliasUrlParams::IndexName(ref index, ref name) => {
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
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a DeleteAliasRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum TasksUrlParams<'a> {
    None,
}
pub struct TasksRequestParams<'a> {
    pub url_params: TasksUrlParams<'a>,
}
impl<'a> TasksRequestParams<'a> {
    pub fn new() -> TasksRequestParams<'a> {
        TasksRequestParams { url_params: TasksUrlParams::None }
    }
}
impl<'a> TasksRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            TasksUrlParams::None => Cow::Borrowed("/_cat/tasks"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a TasksRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum RolloverUrlParams<'a> {
    Alias(Alias<'a>),
    AliasNewIndex(Alias<'a>, NewIndex<'a>),
}
pub struct RolloverRequestParams<'a> {
    pub url_params: RolloverUrlParams<'a>,
    pub body: Body<'a>,
}
impl<'a> RolloverRequestParams<'a> {
    pub fn alias<IAlias: Into<Alias<'a>>, IBody: Into<Body<'a>>>(alias: IAlias,
                                                                 body: IBody)
                                                                 -> RolloverRequestParams<'a> {
        RolloverRequestParams {
            url_params: RolloverUrlParams::Alias(alias.into()),
            body: body.into(),
        }
    }
    pub fn alias_new_index<IAlias: Into<Alias<'a>>,
                           INewIndex: Into<NewIndex<'a>>,
                           IBody: Into<Body<'a>>>
        (alias: IAlias,
         new_index: INewIndex,
         body: IBody)
         -> RolloverRequestParams<'a> {
        RolloverRequestParams {
            url_params: RolloverUrlParams::AliasNewIndex(alias.into(), new_index.into()),
            body: body.into(),
        }
    }
}
impl<'a> RolloverRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            RolloverUrlParams::Alias(ref alias) => {
                let mut url = String::with_capacity(11usize + alias.len());
                url.push_str("/");
                url.push_str(alias.as_ref());
                url.push_str("/_rollover");
                Cow::Owned(url)
            }
            RolloverUrlParams::AliasNewIndex(ref alias, ref new_index) => {
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
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a RolloverRequestParams<'b> {
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
    TaskId(TaskId<'a>),
    TaskId(TaskId<'a>),
}
pub struct ReindexRethrottleRequestParams<'a> {
    pub url_params: ReindexRethrottleUrlParams<'a>,
}
impl<'a> ReindexRethrottleRequestParams<'a> {
    pub fn task_id<ITaskId: Into<TaskId<'a>>>(task_id: ITaskId)
                                              -> ReindexRethrottleRequestParams<'a> {
        ReindexRethrottleRequestParams {
            url_params: ReindexRethrottleUrlParams::TaskId(task_id.into()),
        }
    }
    pub fn task_id<ITaskId: Into<TaskId<'a>>>(task_id: ITaskId)
                                              -> ReindexRethrottleRequestParams<'a> {
        ReindexRethrottleRequestParams {
            url_params: ReindexRethrottleUrlParams::TaskId(task_id.into()),
        }
    }
    pub fn task_id<ITaskId: Into<TaskId<'a>>>(task_id: ITaskId)
                                              -> ReindexRethrottleRequestParams<'a> {
        ReindexRethrottleRequestParams {
            url_params: ReindexRethrottleUrlParams::TaskId(task_id.into()),
        }
    }
}
impl<'a> ReindexRethrottleRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ReindexRethrottleUrlParams::TaskId(ref task_id) => {
                let mut url = String::with_capacity(22usize + task_id.len());
                url.push_str("/_reindex/");
                url.push_str(task_id.as_ref());
                url.push_str("/_rethrottle");
                Cow::Owned(url)
            }
            ReindexRethrottleUrlParams::TaskId(ref task_id) => {
                let mut url = String::with_capacity(30usize + task_id.len());
                url.push_str("/_update_by_query/");
                url.push_str(task_id.as_ref());
                url.push_str("/_rethrottle");
                Cow::Owned(url)
            }
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

pub enum CreateRepositoryUrlParams<'a> {
    Repository(Repository<'a>),
}
pub struct CreateRepositoryRequestParams<'a> {
    pub url_params: CreateRepositoryUrlParams<'a>,
    pub body: Body<'a>,
}
impl<'a> CreateRepositoryRequestParams<'a> {
    pub fn repository<IRepository: Into<Repository<'a>>, IBody: Into<Body<'a>>>
        (repository: IRepository,
         body: IBody)
         -> CreateRepositoryRequestParams<'a> {
        CreateRepositoryRequestParams {
            url_params: CreateRepositoryUrlParams::Repository(repository.into()),
            body: body.into(),
        }
    }
}
impl<'a> CreateRepositoryRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            CreateRepositoryUrlParams::Repository(ref repository) => {
                let mut url = String::with_capacity(11usize + repository.len());
                url.push_str("/_snapshot/");
                url.push_str(repository.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a CreateRepositoryRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum GetUrlParams<'a> {
    Index(Index<'a>),
    IndexFeature(Index<'a>, Feature<'a>),
}
pub struct GetRequestParams<'a> {
    pub url_params: GetUrlParams<'a>,
}
impl<'a> GetRequestParams<'a> {
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> GetRequestParams<'a> {
        GetRequestParams { url_params: GetUrlParams::Index(index.into()) }
    }
    pub fn index_feature<IIndex: Into<Index<'a>>, IFeature: Into<Feature<'a>>>
        (index: IIndex,
         feature: IFeature)
         -> GetRequestParams<'a> {
        GetRequestParams { url_params: GetUrlParams::IndexFeature(index.into(), feature.into()) }
    }
}
impl<'a> GetRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            GetUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(1usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                Cow::Owned(url)
            }
            GetUrlParams::IndexFeature(ref index, ref feature) => {
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
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a GetRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum AnalyzeUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct AnalyzeRequestParams<'a> {
    pub url_params: AnalyzeUrlParams<'a>,
    pub body: Body<'a>,
}
impl<'a> AnalyzeRequestParams<'a> {
    pub fn new<IBody: Into<Body<'a>>>(body: IBody) -> AnalyzeRequestParams<'a> {
        AnalyzeRequestParams {
            url_params: AnalyzeUrlParams::None,
            body: body.into(),
        }
    }
    pub fn index<IIndex: Into<Index<'a>>, IBody: Into<Body<'a>>>(index: IIndex,
                                                                 body: IBody)
                                                                 -> AnalyzeRequestParams<'a> {
        AnalyzeRequestParams {
            url_params: AnalyzeUrlParams::Index(index.into()),
            body: body.into(),
        }
    }
}
impl<'a> AnalyzeRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            AnalyzeUrlParams::None => Cow::Borrowed("/_analyze"),
            AnalyzeUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(10usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_analyze");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a AnalyzeRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum FielddataUrlParams<'a> {
    None,
    Fields(Fields<'a>),
}
pub struct FielddataRequestParams<'a> {
    pub url_params: FielddataUrlParams<'a>,
}
impl<'a> FielddataRequestParams<'a> {
    pub fn new() -> FielddataRequestParams<'a> {
        FielddataRequestParams { url_params: FielddataUrlParams::None }
    }
    pub fn fields<IFields: Into<Fields<'a>>>(fields: IFields) -> FielddataRequestParams<'a> {
        FielddataRequestParams { url_params: FielddataUrlParams::Fields(fields.into()) }
    }
}
impl<'a> FielddataRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            FielddataUrlParams::None => Cow::Borrowed("/_cat/fielddata"),
            FielddataUrlParams::Fields(ref fields) => {
                let mut url = String::with_capacity(16usize + fields.len());
                url.push_str("/_cat/fielddata/");
                url.push_str(fields.as_ref());
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a FielddataRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum SegmentsUrlParams<'a> {
    None,
    Index(Index<'a>),
}
pub struct SegmentsRequestParams<'a> {
    pub url_params: SegmentsUrlParams<'a>,
}
impl<'a> SegmentsRequestParams<'a> {
    pub fn new() -> SegmentsRequestParams<'a> {
        SegmentsRequestParams { url_params: SegmentsUrlParams::None }
    }
    pub fn index<IIndex: Into<Index<'a>>>(index: IIndex) -> SegmentsRequestParams<'a> {
        SegmentsRequestParams { url_params: SegmentsUrlParams::Index(index.into()) }
    }
}
impl<'a> SegmentsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            SegmentsUrlParams::None => Cow::Borrowed("/_segments"),
            SegmentsUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(11usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_segments");
                Cow::Owned(url)
            }
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a SegmentsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum ShrinkUrlParams<'a> {
    IndexTarget(Index<'a>, Target<'a>),
}
pub struct ShrinkRequestParams<'a> {
    pub url_params: ShrinkUrlParams<'a>,
    pub body: Body<'a>,
}
impl<'a> ShrinkRequestParams<'a> {
    pub fn index_target<IIndex: Into<Index<'a>>, ITarget: Into<Target<'a>>, IBody: Into<Body<'a>>>
        (index: IIndex,
         target: ITarget,
         body: IBody)
         -> ShrinkRequestParams<'a> {
        ShrinkRequestParams {
            url_params: ShrinkUrlParams::IndexTarget(index.into(), target.into()),
            body: body.into(),
        }
    }
}
impl<'a> ShrinkRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ShrinkUrlParams::IndexTarget(ref index, ref target) => {
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
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ShrinkRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: Some(&self.body),
        }
    }
}

pub enum ListUrlParams<'a> {
    None,
}
pub struct ListRequestParams<'a> {
    pub url_params: ListUrlParams<'a>,
}
impl<'a> ListRequestParams<'a> {
    pub fn new() -> ListRequestParams<'a> {
        ListRequestParams { url_params: ListUrlParams::None }
    }
}
impl<'a> ListRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ListUrlParams::None => Cow::Borrowed("/_tasks"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ListRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum MasterUrlParams<'a> {
    None,
}
pub struct MasterRequestParams<'a> {
    pub url_params: MasterUrlParams<'a>,
}
impl<'a> MasterRequestParams<'a> {
    pub fn new() -> MasterRequestParams<'a> {
        MasterRequestParams { url_params: MasterUrlParams::None }
    }
}
impl<'a> MasterRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            MasterUrlParams::None => Cow::Borrowed("/_cat/master"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a MasterRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum ExistsTypeUrlParams<'a> {
    IndexType(Index<'a>, Type<'a>),
}
pub struct ExistsTypeRequestParams<'a> {
    pub url_params: ExistsTypeUrlParams<'a>,
}
impl<'a> ExistsTypeRequestParams<'a> {
    pub fn index_ty<IIndex: Into<Index<'a>>, IType: Into<Type<'a>>>
        (index: IIndex,
         ty: IType)
         -> ExistsTypeRequestParams<'a> {
        ExistsTypeRequestParams {
            url_params: ExistsTypeUrlParams::IndexType(index.into(), ty.into()),
        }
    }
}
impl<'a> ExistsTypeRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            ExistsTypeUrlParams::IndexType(ref index, ref ty) => {
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
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a ExistsTypeRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum GetSettingsUrlParams<'a> {
    None,
}
pub struct GetSettingsRequestParams<'a> {
    pub url_params: GetSettingsUrlParams<'a>,
}
impl<'a> GetSettingsRequestParams<'a> {
    pub fn new() -> GetSettingsRequestParams<'a> {
        GetSettingsRequestParams { url_params: GetSettingsUrlParams::None }
    }
}
impl<'a> GetSettingsRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            GetSettingsUrlParams::None => Cow::Borrowed("/_cluster/settings"),
        }
    }
}
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a GetSettingsRequestParams<'b> {
    fn into(self) -> HttpRequest<'a> {
        HttpRequest {
            url: self.url(),
            method: HttpMethod::Post,
            body: None,
        }
    }
}

pub enum InfoUrlParams<'a> {
    None,
    NodeId(NodeId<'a>),
    Metric(Metric<'a>),
    NodeIdMetric(NodeId<'a>, Metric<'a>),
}
pub struct InfoRequestParams<'a> {
    pub url_params: InfoUrlParams<'a>,
}
impl<'a> InfoRequestParams<'a> {
    pub fn new() -> InfoRequestParams<'a> {
        InfoRequestParams { url_params: InfoUrlParams::None }
    }
    pub fn node_id<INodeId: Into<NodeId<'a>>>(node_id: INodeId) -> InfoRequestParams<'a> {
        InfoRequestParams { url_params: InfoUrlParams::NodeId(node_id.into()) }
    }
    pub fn metric<IMetric: Into<Metric<'a>>>(metric: IMetric) -> InfoRequestParams<'a> {
        InfoRequestParams { url_params: InfoUrlParams::Metric(metric.into()) }
    }
    pub fn node_id_metric<INodeId: Into<NodeId<'a>>, IMetric: Into<Metric<'a>>>
        (node_id: INodeId,
         metric: IMetric)
         -> InfoRequestParams<'a> {
        InfoRequestParams { url_params: InfoUrlParams::NodeIdMetric(node_id.into(), metric.into()) }
    }
}
impl<'a> InfoRequestParams<'a> {
    pub fn url(&'a self) -> Cow<'a, str> {
        match self.url_params {
            InfoUrlParams::None => Cow::Borrowed("/_nodes"),
            InfoUrlParams::NodeId(ref node_id) => {
                let mut url = String::with_capacity(8usize + node_id.len());
                url.push_str("/_nodes/");
                url.push_str(node_id.as_ref());
                Cow::Owned(url)
            }
            InfoUrlParams::Metric(ref metric) => {
                let mut url = String::with_capacity(8usize + metric.len());
                url.push_str("/_nodes/");
                url.push_str(metric.as_ref());
                Cow::Owned(url)
            }
            InfoUrlParams::NodeIdMetric(ref node_id, ref metric) => {
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
impl<'a, 'b: 'a> Into<HttpRequest<'a>> for &'a InfoRequestParams<'b> {
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

pub struct Indices<'a>(pub Cow<'a, str>);
impl<'a> From<&'a str> for Indices<'a> {
    fn from(value: &'a str) -> Indices<'a> {
        Indices(Cow::Borrowed(value))
    }
}
impl<'a> From<String> for Indices<'a> {
    fn from(value: String) -> Indices<'a> {
        Indices(Cow::Owned(value))
    }
}
impl<'a> ::std::ops::Deref for Indices<'a> {
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
