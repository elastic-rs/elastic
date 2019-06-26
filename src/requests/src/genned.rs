/*This code is automatically generated
*/
pub mod endpoints {
    use super::{
        http::*,
        params::*,
    };

    #[derive(Debug, PartialEq, Clone)]
    enum IndicesUpgradeUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesUpgradeUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesUpgradeUrlParams::None => UrlPath::from("/_upgrade"),
                IndicesUpgradeUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(10usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_upgrade");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_upgrade`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-upgrade.html)"]
    pub struct IndicesUpgradeRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesUpgradeRequest<'a, B> {
        #[doc = "Request to: `/_upgrade`"]
        pub fn new(body: B) -> Self {
            IndicesUpgradeRequest {
                url: IndicesUpgradeUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_upgrade`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesUpgradeRequest {
                url: IndicesUpgradeUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for IndicesUpgradeRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SnapshotStatusUrlParams<'a> {
        None,
        Repository(Repository<'a>),
        RepositorySnapshot(Repository<'a>, Snapshot<'a>),
    }
    impl<'a> SnapshotStatusUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                SnapshotStatusUrlParams::None => UrlPath::from("/_snapshot/_status"),
                SnapshotStatusUrlParams::Repository(ref repository) => {
                    let mut url = String::with_capacity(19usize + repository.len());
                    url.push_str("/_snapshot/");
                    url.push_str(repository.as_ref());
                    url.push_str("/_status");
                    UrlPath::from(url)
                }
                SnapshotStatusUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
                    let mut url =
                        String::with_capacity(20usize + repository.len() + snapshot.len());
                    url.push_str("/_snapshot/");
                    url.push_str(repository.as_ref());
                    url.push_str("/");
                    url.push_str(snapshot.as_ref());
                    url.push_str("/_status");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_snapshot/_status`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html)"]
    pub struct SnapshotStatusRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> SnapshotStatusRequest<'a> {
        #[doc = "Request to: `/_snapshot/_status`"]
        pub fn new() -> Self {
            SnapshotStatusRequest {
                url: SnapshotStatusUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/_snapshot/{repository}/_status`"]
        pub fn for_repository<IRepository>(repository: IRepository) -> Self
        where
            IRepository: Into<Repository<'a>>,
        {
            SnapshotStatusRequest {
                url: SnapshotStatusUrlParams::Repository(repository.into()).url(),
            }
        }
        #[doc = "Request to: `/_snapshot/{repository}/{snapshot}/_status`"]
        pub fn for_repository_snapshot<IRepository, ISnapshot>(
            repository: IRepository,
            snapshot: ISnapshot,
        ) -> Self
        where
            IRepository: Into<Repository<'a>>,
            ISnapshot: Into<Snapshot<'a>>,
        {
            SnapshotStatusRequest {
                url: SnapshotStatusUrlParams::RepositorySnapshot(
                    repository.into(),
                    snapshot.into(),
                )
                .url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for SnapshotStatusRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesGetFieldMappingUrlParams<'a> {
        Fields(Fields<'a>),
        IndexFields(Index<'a>, Fields<'a>),
        IndexTypeFields(Index<'a>, Type<'a>, Fields<'a>),
        TypeFields(Type<'a>, Fields<'a>),
    }
    impl<'a> IndicesGetFieldMappingUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesGetFieldMappingUrlParams::Fields(ref fields) => {
                    let mut url = String::with_capacity(16usize + fields.len());
                    url.push_str("/_mapping/field/");
                    url.push_str(fields.as_ref());
                    UrlPath::from(url)
                }
                IndicesGetFieldMappingUrlParams::IndexFields(ref index, ref fields) => {
                    let mut url = String::with_capacity(17usize + index.len() + fields.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_mapping/field/");
                    url.push_str(fields.as_ref());
                    UrlPath::from(url)
                }
                IndicesGetFieldMappingUrlParams::IndexTypeFields(ref index, ref ty, ref fields) => {
                    let mut url =
                        String::with_capacity(18usize + index.len() + ty.len() + fields.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_mapping/");
                    url.push_str(ty.as_ref());
                    url.push_str("/field/");
                    url.push_str(fields.as_ref());
                    UrlPath::from(url)
                }
                IndicesGetFieldMappingUrlParams::TypeFields(ref ty, ref fields) => {
                    let mut url = String::with_capacity(17usize + ty.len() + fields.len());
                    url.push_str("/_mapping/");
                    url.push_str(ty.as_ref());
                    url.push_str("/field/");
                    url.push_str(fields.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_mapping/field/{fields}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-field-mapping.html)"]
    pub struct IndicesGetFieldMappingRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> IndicesGetFieldMappingRequest<'a> {
        #[doc = "Request to: `/_mapping/field/{fields}`"]
        pub fn for_fields<IFields>(fields: IFields) -> Self
        where
            IFields: Into<Fields<'a>>,
        {
            IndicesGetFieldMappingRequest {
                url: IndicesGetFieldMappingUrlParams::Fields(fields.into()).url(),
            }
        }
        #[doc = "Request to: `/{index}/_mapping/field/{fields}`"]
        pub fn for_index_fields<IIndex, IFields>(index: IIndex, fields: IFields) -> Self
        where
            IIndex: Into<Index<'a>>,
            IFields: Into<Fields<'a>>,
        {
            IndicesGetFieldMappingRequest {
                url: IndicesGetFieldMappingUrlParams::IndexFields(index.into(), fields.into())
                    .url(),
            }
        }
        #[doc = "Request to: `/{index}/_mapping/{type}/field/{fields}`"]
        pub fn for_index_ty_fields<IIndex, IType, IFields>(
            index: IIndex,
            ty: IType,
            fields: IFields,
        ) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IFields: Into<Fields<'a>>,
        {
            IndicesGetFieldMappingRequest {
                url: IndicesGetFieldMappingUrlParams::IndexTypeFields(
                    index.into(),
                    ty.into(),
                    fields.into(),
                )
                .url(),
            }
        }
        #[doc = "Request to: `/_mapping/{type}/field/{fields}`"]
        pub fn for_ty_fields<IType, IFields>(ty: IType, fields: IFields) -> Self
        where
            IType: Into<Type<'a>>,
            IFields: Into<Fields<'a>>,
        {
            IndicesGetFieldMappingRequest {
                url: IndicesGetFieldMappingUrlParams::TypeFields(ty.into(), fields.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for IndicesGetFieldMappingRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesUpdateAliasesUrlParams {
        None,
    }
    impl IndicesUpdateAliasesUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                IndicesUpdateAliasesUrlParams::None => UrlPath::from("/_aliases"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_aliases`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html)"]
    pub struct IndicesUpdateAliasesRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesUpdateAliasesRequest<'a, B> {
        #[doc = "Request to: `/_aliases`"]
        pub fn new(body: B) -> Self {
            IndicesUpdateAliasesRequest {
                url: IndicesUpdateAliasesUrlParams::None.url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for IndicesUpdateAliasesRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ScrollUrlParams<'a> {
        None,
        ScrollId(ScrollId<'a>),
    }
    impl<'a> ScrollUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                ScrollUrlParams::None => UrlPath::from("/_search/scroll"),
                ScrollUrlParams::ScrollId(ref scroll_id) => {
                    let mut url = String::with_capacity(16usize + scroll_id.len());
                    url.push_str("/_search/scroll/");
                    url.push_str(scroll_id.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_search/scroll`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/search-request-scroll.html)"]
    pub struct ScrollRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> ScrollRequest<'a, B> {
        #[doc = "Request to: `/_search/scroll`"]
        pub fn new(body: B) -> Self {
            ScrollRequest {
                url: ScrollUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/_search/scroll/{scroll_id}`"]
        pub fn for_scroll_id<IScrollId>(scroll_id: IScrollId, body: B) -> Self
        where
            IScrollId: Into<ScrollId<'a>>,
        {
            ScrollRequest {
                url: ScrollUrlParams::ScrollId(scroll_id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for ScrollRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesGetUrlParams<'a> {
        Index(Index<'a>),
    }
    impl<'a> IndicesGetUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesGetUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(1usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /{index}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-index.html)"]
    pub struct IndicesGetRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> IndicesGetRequest<'a> {
        #[doc = "Request to: `/{index}`"]
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesGetRequest {
                url: IndicesGetUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for IndicesGetRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum RankEvalUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> RankEvalUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                RankEvalUrlParams::None => UrlPath::from("/_rank_eval"),
                RankEvalUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(12usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_rank_eval");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_rank_eval`\n\n[Elasticsearch Documentation](https://www.elastic.co/guide/en/elasticsearch/reference/master/search-rank-eval.html)"]
    pub struct RankEvalRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> RankEvalRequest<'a, B> {
        #[doc = "Request to: `/_rank_eval`"]
        pub fn new(body: B) -> Self {
            RankEvalRequest {
                url: RankEvalUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_rank_eval`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            RankEvalRequest {
                url: RankEvalUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for RankEvalRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum TasksListUrlParams {
        None,
    }
    impl TasksListUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                TasksListUrlParams::None => UrlPath::from("/_tasks"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_tasks`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html)"]
    pub struct TasksListRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> TasksListRequest<'a> {
        #[doc = "Request to: `/_tasks`"]
        pub fn new() -> Self {
            TasksListRequest {
                url: TasksListUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for TasksListRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum MsearchUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> MsearchUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                MsearchUrlParams::None => UrlPath::from("/_msearch"),
                MsearchUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(10usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_msearch");
                    UrlPath::from(url)
                }
                MsearchUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(11usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_msearch");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_msearch`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/search-multi-search.html)"]
    pub struct MsearchRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> MsearchRequest<'a, B> {
        #[doc = "Request to: `/_msearch`"]
        pub fn new(body: B) -> Self {
            MsearchRequest {
                url: MsearchUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_msearch`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            MsearchRequest {
                url: MsearchUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/{type}/_msearch`"]
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            MsearchRequest {
                url: MsearchUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for MsearchRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum FieldCapsUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> FieldCapsUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                FieldCapsUrlParams::None => UrlPath::from("/_field_caps"),
                FieldCapsUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(13usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_field_caps");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_field_caps`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/search-field-caps.html)"]
    pub struct FieldCapsRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> FieldCapsRequest<'a, B> {
        #[doc = "Request to: `/_field_caps`"]
        pub fn new(body: B) -> Self {
            FieldCapsRequest {
                url: FieldCapsUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_field_caps`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            FieldCapsRequest {
                url: FieldCapsUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for FieldCapsRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum TasksCancelUrlParams<'a> {
        None,
        TaskId(TaskId<'a>),
    }
    impl<'a> TasksCancelUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                TasksCancelUrlParams::None => UrlPath::from("/_tasks/_cancel"),
                TasksCancelUrlParams::TaskId(ref task_id) => {
                    let mut url = String::with_capacity(16usize + task_id.len());
                    url.push_str("/_tasks/");
                    url.push_str(task_id.as_ref());
                    url.push_str("/_cancel");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_tasks`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html)"]
    pub struct TasksCancelRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> TasksCancelRequest<'a, B> {
        #[doc = "Request to: `/_tasks/_cancel`"]
        pub fn new(body: B) -> Self {
            TasksCancelRequest {
                url: TasksCancelUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/_tasks/{task_id}/_cancel`"]
        pub fn for_task_id<ITaskId>(task_id: ITaskId, body: B) -> Self
        where
            ITaskId: Into<TaskId<'a>>,
        {
            TasksCancelRequest {
                url: TasksCancelUrlParams::TaskId(task_id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for TasksCancelRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesRolloverUrlParams<'a> {
        Alias(Alias<'a>),
        AliasNewIndex(Alias<'a>, NewIndex<'a>),
    }
    impl<'a> IndicesRolloverUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesRolloverUrlParams::Alias(ref alias) => {
                    let mut url = String::with_capacity(11usize + alias.len());
                    url.push_str("/");
                    url.push_str(alias.as_ref());
                    url.push_str("/_rollover");
                    UrlPath::from(url)
                }
                IndicesRolloverUrlParams::AliasNewIndex(ref alias, ref new_index) => {
                    let mut url = String::with_capacity(12usize + alias.len() + new_index.len());
                    url.push_str("/");
                    url.push_str(alias.as_ref());
                    url.push_str("/_rollover/");
                    url.push_str(new_index.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /{alias}/_rollover`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-rollover-index.html)"]
    pub struct IndicesRolloverRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesRolloverRequest<'a, B> {
        #[doc = "Request to: `/{alias}/_rollover`"]
        pub fn for_alias<IAlias>(alias: IAlias, body: B) -> Self
        where
            IAlias: Into<Alias<'a>>,
        {
            IndicesRolloverRequest {
                url: IndicesRolloverUrlParams::Alias(alias.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{alias}/_rollover/{new_index}`"]
        pub fn for_alias_new_index<IAlias, INewIndex>(
            alias: IAlias,
            new_index: INewIndex,
            body: B,
        ) -> Self
        where
            IAlias: Into<Alias<'a>>,
            INewIndex: Into<NewIndex<'a>>,
        {
            IndicesRolloverRequest {
                url: IndicesRolloverUrlParams::AliasNewIndex(alias.into(), new_index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for IndicesRolloverRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ScriptsPainlessExecuteUrlParams {
        None,
    }
    impl ScriptsPainlessExecuteUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                ScriptsPainlessExecuteUrlParams::None => {
                    UrlPath::from("/_scripts/painless/_execute")
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_scripts/painless/_execute`\n\n[Elasticsearch Documentation](https://www.elastic.co/guide/en/elasticsearch/painless/master/painless-execute-api.html)"]
    pub struct ScriptsPainlessExecuteRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> ScriptsPainlessExecuteRequest<'a, B> {
        #[doc = "Request to: `/_scripts/painless/_execute`"]
        pub fn new(body: B) -> Self {
            ScriptsPainlessExecuteRequest {
                url: ScriptsPainlessExecuteUrlParams::None.url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for ScriptsPainlessExecuteRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatSnapshotsUrlParams<'a> {
        None,
        Repository(Repository<'a>),
    }
    impl<'a> CatSnapshotsUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                CatSnapshotsUrlParams::None => UrlPath::from("/_cat/snapshots"),
                CatSnapshotsUrlParams::Repository(ref repository) => {
                    let mut url = String::with_capacity(16usize + repository.len());
                    url.push_str("/_cat/snapshots/");
                    url.push_str(repository.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cat/snapshots`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-snapshots.html)"]
    pub struct CatSnapshotsRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> CatSnapshotsRequest<'a> {
        #[doc = "Request to: `/_cat/snapshots`"]
        pub fn new() -> Self {
            CatSnapshotsRequest {
                url: CatSnapshotsUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/_cat/snapshots/{repository}`"]
        pub fn for_repository<IRepository>(repository: IRepository) -> Self
        where
            IRepository: Into<Repository<'a>>,
        {
            CatSnapshotsRequest {
                url: CatSnapshotsUrlParams::Repository(repository.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for CatSnapshotsRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum RenderSearchTemplateUrlParams<'a> {
        None,
        Id(Id<'a>),
    }
    impl<'a> RenderSearchTemplateUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                RenderSearchTemplateUrlParams::None => UrlPath::from("/_render/template"),
                RenderSearchTemplateUrlParams::Id(ref id) => {
                    let mut url = String::with_capacity(18usize + id.len());
                    url.push_str("/_render/template/");
                    url.push_str(id.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_render/template`\n\n[Elasticsearch Documentation](http://www.elasticsearch.org/guide/en/elasticsearch/reference/master/search-template.html)"]
    pub struct RenderSearchTemplateRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> RenderSearchTemplateRequest<'a, B> {
        #[doc = "Request to: `/_render/template`"]
        pub fn new(body: B) -> Self {
            RenderSearchTemplateRequest {
                url: RenderSearchTemplateUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/_render/template/{id}`"]
        pub fn for_id<IId>(id: IId, body: B) -> Self
        where
            IId: Into<Id<'a>>,
        {
            RenderSearchTemplateRequest {
                url: RenderSearchTemplateUrlParams::Id(id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for RenderSearchTemplateRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesSegmentsUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesSegmentsUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesSegmentsUrlParams::None => UrlPath::from("/_segments"),
                IndicesSegmentsUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(11usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_segments");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_segments`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-segments.html)"]
    pub struct IndicesSegmentsRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> IndicesSegmentsRequest<'a> {
        #[doc = "Request to: `/_segments`"]
        pub fn new() -> Self {
            IndicesSegmentsRequest {
                url: IndicesSegmentsUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/{index}/_segments`"]
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesSegmentsRequest {
                url: IndicesSegmentsUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for IndicesSegmentsRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatMasterUrlParams {
        None,
    }
    impl CatMasterUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                CatMasterUrlParams::None => UrlPath::from("/_cat/master"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cat/master`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-master.html)"]
    pub struct CatMasterRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> CatMasterRequest<'a> {
        #[doc = "Request to: `/_cat/master`"]
        pub fn new() -> Self {
            CatMasterRequest {
                url: CatMasterUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for CatMasterRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatFielddataUrlParams<'a> {
        None,
        Fields(Fields<'a>),
    }
    impl<'a> CatFielddataUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                CatFielddataUrlParams::None => UrlPath::from("/_cat/fielddata"),
                CatFielddataUrlParams::Fields(ref fields) => {
                    let mut url = String::with_capacity(16usize + fields.len());
                    url.push_str("/_cat/fielddata/");
                    url.push_str(fields.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cat/fielddata`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-fielddata.html)"]
    pub struct CatFielddataRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> CatFielddataRequest<'a> {
        #[doc = "Request to: `/_cat/fielddata`"]
        pub fn new() -> Self {
            CatFielddataRequest {
                url: CatFielddataUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/_cat/fielddata/{fields}`"]
        pub fn for_fields<IFields>(fields: IFields) -> Self
        where
            IFields: Into<Fields<'a>>,
        {
            CatFielddataRequest {
                url: CatFielddataUrlParams::Fields(fields.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for CatFielddataRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SqlQueryUrlParams {
        None,
    }
    impl SqlQueryUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                SqlQueryUrlParams::None => UrlPath::from("/_xpack/sql"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_xpack/sql`\n\n[Elasticsearch Documentation](Execute SQL)"]
    pub struct SqlQueryRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> SqlQueryRequest<'a, B> {
        #[doc = "Request to: `/_xpack/sql`"]
        pub fn new(body: B) -> Self {
            SqlQueryRequest {
                url: SqlQueryUrlParams::None.url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for SqlQueryRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum UpdateUrlParams<'a> {
        IndexId(Index<'a>, Id<'a>),
        IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
    }
    impl<'a> UpdateUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                UpdateUrlParams::IndexId(ref index, ref id) => {
                    let mut url = String::with_capacity(10usize + index.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_update/");
                    url.push_str(id.as_ref());
                    UrlPath::from(url)
                }
                UpdateUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                    let mut url =
                        String::with_capacity(11usize + index.len() + ty.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    url.push_str("/_update");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /{index}/_update/{id}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-update.html)"]
    pub struct UpdateRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> UpdateRequest<'a, B> {
        #[doc = "Request to: `/{index}/_update/{id}`"]
        pub fn for_index_id<IIndex, IId>(index: IIndex, id: IId, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IId: Into<Id<'a>>,
        {
            UpdateRequest {
                url: UpdateUrlParams::IndexId(index.into(), id.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/{type}/{id}/_update`"]
        pub fn for_index_ty_id<IIndex, IType, IId>(
            index: IIndex,
            ty: IType,
            id: IId,
            body: B,
        ) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IId: Into<Id<'a>>,
        {
            UpdateRequest {
                url: UpdateUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for UpdateRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesExistsTypeUrlParams<'a> {
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> IndicesExistsTypeUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesExistsTypeUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(11usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_mapping/");
                    url.push_str(ty.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Head: /{index}/_mapping/{type}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-types-exists.html)"]
    pub struct IndicesExistsTypeRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> IndicesExistsTypeRequest<'a> {
        #[doc = "Request to: `/{index}/_mapping/{type}`"]
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            IndicesExistsTypeRequest {
                url: IndicesExistsTypeUrlParams::IndexType(index.into(), ty.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for IndicesExistsTypeRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::HEAD,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesGetMappingUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
        Type(Type<'a>),
    }
    impl<'a> IndicesGetMappingUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesGetMappingUrlParams::None => UrlPath::from("/_mapping"),
                IndicesGetMappingUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(10usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_mapping");
                    UrlPath::from(url)
                }
                IndicesGetMappingUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(11usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_mapping/");
                    url.push_str(ty.as_ref());
                    UrlPath::from(url)
                }
                IndicesGetMappingUrlParams::Type(ref ty) => {
                    let mut url = String::with_capacity(10usize + ty.len());
                    url.push_str("/_mapping/");
                    url.push_str(ty.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_mapping`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-mapping.html)"]
    pub struct IndicesGetMappingRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> IndicesGetMappingRequest<'a> {
        #[doc = "Request to: `/_mapping`"]
        pub fn new() -> Self {
            IndicesGetMappingRequest {
                url: IndicesGetMappingUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/{index}/_mapping`"]
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesGetMappingRequest {
                url: IndicesGetMappingUrlParams::Index(index.into()).url(),
            }
        }
        #[doc = "Request to: `/{index}/_mapping/{type}`"]
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            IndicesGetMappingRequest {
                url: IndicesGetMappingUrlParams::IndexType(index.into(), ty.into()).url(),
            }
        }
        #[doc = "Request to: `/_mapping/{type}`"]
        pub fn for_ty<IType>(ty: IType) -> Self
        where
            IType: Into<Type<'a>>,
        {
            IndicesGetMappingRequest {
                url: IndicesGetMappingUrlParams::Type(ty.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for IndicesGetMappingRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesRefreshUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesRefreshUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesRefreshUrlParams::None => UrlPath::from("/_refresh"),
                IndicesRefreshUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(10usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_refresh");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_refresh`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-refresh.html)"]
    pub struct IndicesRefreshRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesRefreshRequest<'a, B> {
        #[doc = "Request to: `/_refresh`"]
        pub fn new(body: B) -> Self {
            IndicesRefreshRequest {
                url: IndicesRefreshUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_refresh`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesRefreshRequest {
                url: IndicesRefreshUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for IndicesRefreshRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum NodesStatsUrlParams<'a> {
        None,
        Metric(Metric<'a>),
        MetricIndexMetric(Metric<'a>, IndexMetric<'a>),
        NodeId(NodeId<'a>),
        NodeIdMetric(NodeId<'a>, Metric<'a>),
        NodeIdMetricIndexMetric(NodeId<'a>, Metric<'a>, IndexMetric<'a>),
    }
    impl<'a> NodesStatsUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                NodesStatsUrlParams::None => UrlPath::from("/_nodes/stats"),
                NodesStatsUrlParams::Metric(ref metric) => {
                    let mut url = String::with_capacity(14usize + metric.len());
                    url.push_str("/_nodes/stats/");
                    url.push_str(metric.as_ref());
                    UrlPath::from(url)
                }
                NodesStatsUrlParams::MetricIndexMetric(ref metric, ref index_metric) => {
                    let mut url =
                        String::with_capacity(15usize + metric.len() + index_metric.len());
                    url.push_str("/_nodes/stats/");
                    url.push_str(metric.as_ref());
                    url.push_str("/");
                    url.push_str(index_metric.as_ref());
                    UrlPath::from(url)
                }
                NodesStatsUrlParams::NodeId(ref node_id) => {
                    let mut url = String::with_capacity(14usize + node_id.len());
                    url.push_str("/_nodes/");
                    url.push_str(node_id.as_ref());
                    url.push_str("/stats");
                    UrlPath::from(url)
                }
                NodesStatsUrlParams::NodeIdMetric(ref node_id, ref metric) => {
                    let mut url = String::with_capacity(15usize + node_id.len() + metric.len());
                    url.push_str("/_nodes/");
                    url.push_str(node_id.as_ref());
                    url.push_str("/stats/");
                    url.push_str(metric.as_ref());
                    UrlPath::from(url)
                }
                NodesStatsUrlParams::NodeIdMetricIndexMetric(
                    ref node_id,
                    ref metric,
                    ref index_metric,
                ) => {
                    let mut url = String::with_capacity(
                        16usize + node_id.len() + metric.len() + index_metric.len(),
                    );
                    url.push_str("/_nodes/");
                    url.push_str(node_id.as_ref());
                    url.push_str("/stats/");
                    url.push_str(metric.as_ref());
                    url.push_str("/");
                    url.push_str(index_metric.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_nodes/stats`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-stats.html)"]
    pub struct NodesStatsRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> NodesStatsRequest<'a> {
        #[doc = "Request to: `/_nodes/stats`"]
        pub fn new() -> Self {
            NodesStatsRequest {
                url: NodesStatsUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/_nodes/stats/{metric}`"]
        pub fn for_metric<IMetric>(metric: IMetric) -> Self
        where
            IMetric: Into<Metric<'a>>,
        {
            NodesStatsRequest {
                url: NodesStatsUrlParams::Metric(metric.into()).url(),
            }
        }
        #[doc = "Request to: `/_nodes/stats/{metric}/{index_metric}`"]
        pub fn for_metric_index_metric<IMetric, IIndexMetric>(
            metric: IMetric,
            index_metric: IIndexMetric,
        ) -> Self
        where
            IMetric: Into<Metric<'a>>,
            IIndexMetric: Into<IndexMetric<'a>>,
        {
            NodesStatsRequest {
                url: NodesStatsUrlParams::MetricIndexMetric(metric.into(), index_metric.into())
                    .url(),
            }
        }
        #[doc = "Request to: `/_nodes/{node_id}/stats`"]
        pub fn for_node_id<INodeId>(node_id: INodeId) -> Self
        where
            INodeId: Into<NodeId<'a>>,
        {
            NodesStatsRequest {
                url: NodesStatsUrlParams::NodeId(node_id.into()).url(),
            }
        }
        #[doc = "Request to: `/_nodes/{node_id}/stats/{metric}`"]
        pub fn for_node_id_metric<INodeId, IMetric>(node_id: INodeId, metric: IMetric) -> Self
        where
            INodeId: Into<NodeId<'a>>,
            IMetric: Into<Metric<'a>>,
        {
            NodesStatsRequest {
                url: NodesStatsUrlParams::NodeIdMetric(node_id.into(), metric.into()).url(),
            }
        }
        #[doc = "Request to: `/_nodes/{node_id}/stats/{metric}/{index_metric}`"]
        pub fn for_node_id_metric_index_metric<INodeId, IMetric, IIndexMetric>(
            node_id: INodeId,
            metric: IMetric,
            index_metric: IIndexMetric,
        ) -> Self
        where
            INodeId: Into<NodeId<'a>>,
            IMetric: Into<Metric<'a>>,
            IIndexMetric: Into<IndexMetric<'a>>,
        {
            NodesStatsRequest {
                url: NodesStatsUrlParams::NodeIdMetricIndexMetric(
                    node_id.into(),
                    metric.into(),
                    index_metric.into(),
                )
                .url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for NodesStatsRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ReindexUrlParams {
        None,
    }
    impl ReindexUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                ReindexUrlParams::None => UrlPath::from("/_reindex"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_reindex`\n\n[Elasticsearch Documentation](https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-reindex.html)"]
    pub struct ReindexRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> ReindexRequest<'a, B> {
        #[doc = "Request to: `/_reindex`"]
        pub fn new(body: B) -> Self {
            ReindexRequest {
                url: ReindexUrlParams::None.url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for ReindexRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ClusterStatsUrlParams<'a> {
        None,
        NodeId(NodeId<'a>),
    }
    impl<'a> ClusterStatsUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                ClusterStatsUrlParams::None => UrlPath::from("/_cluster/stats"),
                ClusterStatsUrlParams::NodeId(ref node_id) => {
                    let mut url = String::with_capacity(22usize + node_id.len());
                    url.push_str("/_cluster/stats/nodes/");
                    url.push_str(node_id.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cluster/stats`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-stats.html)"]
    pub struct ClusterStatsRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> ClusterStatsRequest<'a> {
        #[doc = "Request to: `/_cluster/stats`"]
        pub fn new() -> Self {
            ClusterStatsRequest {
                url: ClusterStatsUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/_cluster/stats/nodes/{node_id}`"]
        pub fn for_node_id<INodeId>(node_id: INodeId) -> Self
        where
            INodeId: Into<NodeId<'a>>,
        {
            ClusterStatsRequest {
                url: ClusterStatsUrlParams::NodeId(node_id.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for ClusterStatsRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesGetAliasUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexName(Index<'a>, Name<'a>),
        Name(Name<'a>),
    }
    impl<'a> IndicesGetAliasUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesGetAliasUrlParams::None => UrlPath::from("/_alias"),
                IndicesGetAliasUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(8usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_alias");
                    UrlPath::from(url)
                }
                IndicesGetAliasUrlParams::IndexName(ref index, ref name) => {
                    let mut url = String::with_capacity(9usize + index.len() + name.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_alias/");
                    url.push_str(name.as_ref());
                    UrlPath::from(url)
                }
                IndicesGetAliasUrlParams::Name(ref name) => {
                    let mut url = String::with_capacity(8usize + name.len());
                    url.push_str("/_alias/");
                    url.push_str(name.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_alias/`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html)"]
    pub struct IndicesGetAliasRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> IndicesGetAliasRequest<'a> {
        #[doc = "Request to: `/_alias`"]
        pub fn new() -> Self {
            IndicesGetAliasRequest {
                url: IndicesGetAliasUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/{index}/_alias`"]
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesGetAliasRequest {
                url: IndicesGetAliasUrlParams::Index(index.into()).url(),
            }
        }
        #[doc = "Request to: `/{index}/_alias/{name}`"]
        pub fn for_index_name<IIndex, IName>(index: IIndex, name: IName) -> Self
        where
            IIndex: Into<Index<'a>>,
            IName: Into<Name<'a>>,
        {
            IndicesGetAliasRequest {
                url: IndicesGetAliasUrlParams::IndexName(index.into(), name.into()).url(),
            }
        }
        #[doc = "Request to: `/_alias/{name}`"]
        pub fn for_name<IName>(name: IName) -> Self
        where
            IName: Into<Name<'a>>,
        {
            IndicesGetAliasRequest {
                url: IndicesGetAliasUrlParams::Name(name.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for IndicesGetAliasRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SearchTemplateUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> SearchTemplateUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                SearchTemplateUrlParams::None => UrlPath::from("/_search/template"),
                SearchTemplateUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(18usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_search/template");
                    UrlPath::from(url)
                }
                SearchTemplateUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(19usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_search/template");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_search/template`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/current/search-template.html)"]
    pub struct SearchTemplateRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> SearchTemplateRequest<'a, B> {
        #[doc = "Request to: `/_search/template`"]
        pub fn new(body: B) -> Self {
            SearchTemplateRequest {
                url: SearchTemplateUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_search/template`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            SearchTemplateRequest {
                url: SearchTemplateUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/{type}/_search/template`"]
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            SearchTemplateRequest {
                url: SearchTemplateUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for SearchTemplateRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatNodesUrlParams {
        None,
    }
    impl CatNodesUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                CatNodesUrlParams::None => UrlPath::from("/_cat/nodes"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cat/nodes`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-nodes.html)"]
    pub struct CatNodesRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> CatNodesRequest<'a> {
        #[doc = "Request to: `/_cat/nodes`"]
        pub fn new() -> Self {
            CatNodesRequest {
                url: CatNodesUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for CatNodesRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum MsearchTemplateUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> MsearchTemplateUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                MsearchTemplateUrlParams::None => UrlPath::from("/_msearch/template"),
                MsearchTemplateUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(19usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_msearch/template");
                    UrlPath::from(url)
                }
                MsearchTemplateUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(20usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_msearch/template");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_msearch/template`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/current/search-multi-search.html)"]
    pub struct MsearchTemplateRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> MsearchTemplateRequest<'a, B> {
        #[doc = "Request to: `/_msearch/template`"]
        pub fn new(body: B) -> Self {
            MsearchTemplateRequest {
                url: MsearchTemplateUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_msearch/template`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            MsearchTemplateRequest {
                url: MsearchTemplateUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/{type}/_msearch/template`"]
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            MsearchTemplateRequest {
                url: MsearchTemplateUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for MsearchTemplateRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesForcemergeUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesForcemergeUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesForcemergeUrlParams::None => UrlPath::from("/_forcemerge"),
                IndicesForcemergeUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(13usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_forcemerge");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_forcemerge`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-forcemerge.html)"]
    pub struct IndicesForcemergeRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesForcemergeRequest<'a, B> {
        #[doc = "Request to: `/_forcemerge`"]
        pub fn new(body: B) -> Self {
            IndicesForcemergeRequest {
                url: IndicesForcemergeUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_forcemerge`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesForcemergeRequest {
                url: IndicesForcemergeUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for IndicesForcemergeRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SnapshotCreateUrlParams<'a> {
        RepositorySnapshot(Repository<'a>, Snapshot<'a>),
    }
    impl<'a> SnapshotCreateUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                SnapshotCreateUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
                    let mut url =
                        String::with_capacity(12usize + repository.len() + snapshot.len());
                    url.push_str("/_snapshot/");
                    url.push_str(repository.as_ref());
                    url.push_str("/");
                    url.push_str(snapshot.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_snapshot/{repository}/{snapshot}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html)"]
    pub struct SnapshotCreateRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> SnapshotCreateRequest<'a, B> {
        #[doc = "Request to: `/_snapshot/{repository}/{snapshot}`"]
        pub fn for_repository_snapshot<IRepository, ISnapshot>(
            repository: IRepository,
            snapshot: ISnapshot,
            body: B,
        ) -> Self
        where
            IRepository: Into<Repository<'a>>,
            ISnapshot: Into<Snapshot<'a>>,
        {
            SnapshotCreateRequest {
                url: SnapshotCreateUrlParams::RepositorySnapshot(
                    repository.into(),
                    snapshot.into(),
                )
                .url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for SnapshotCreateRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesFlushUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesFlushUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesFlushUrlParams::None => UrlPath::from("/_flush"),
                IndicesFlushUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(8usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_flush");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_flush`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-flush.html)"]
    pub struct IndicesFlushRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesFlushRequest<'a, B> {
        #[doc = "Request to: `/_flush`"]
        pub fn new(body: B) -> Self {
            IndicesFlushRequest {
                url: IndicesFlushUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_flush`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesFlushRequest {
                url: IndicesFlushUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for IndicesFlushRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesGetSettingsUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexName(Index<'a>, Name<'a>),
        Name(Name<'a>),
    }
    impl<'a> IndicesGetSettingsUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesGetSettingsUrlParams::None => UrlPath::from("/_settings"),
                IndicesGetSettingsUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(11usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_settings");
                    UrlPath::from(url)
                }
                IndicesGetSettingsUrlParams::IndexName(ref index, ref name) => {
                    let mut url = String::with_capacity(12usize + index.len() + name.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_settings/");
                    url.push_str(name.as_ref());
                    UrlPath::from(url)
                }
                IndicesGetSettingsUrlParams::Name(ref name) => {
                    let mut url = String::with_capacity(11usize + name.len());
                    url.push_str("/_settings/");
                    url.push_str(name.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_settings`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-settings.html)"]
    pub struct IndicesGetSettingsRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> IndicesGetSettingsRequest<'a> {
        #[doc = "Request to: `/_settings`"]
        pub fn new() -> Self {
            IndicesGetSettingsRequest {
                url: IndicesGetSettingsUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/{index}/_settings`"]
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesGetSettingsRequest {
                url: IndicesGetSettingsUrlParams::Index(index.into()).url(),
            }
        }
        #[doc = "Request to: `/{index}/_settings/{name}`"]
        pub fn for_index_name<IIndex, IName>(index: IIndex, name: IName) -> Self
        where
            IIndex: Into<Index<'a>>,
            IName: Into<Name<'a>>,
        {
            IndicesGetSettingsRequest {
                url: IndicesGetSettingsUrlParams::IndexName(index.into(), name.into()).url(),
            }
        }
        #[doc = "Request to: `/_settings/{name}`"]
        pub fn for_name<IName>(name: IName) -> Self
        where
            IName: Into<Name<'a>>,
        {
            IndicesGetSettingsRequest {
                url: IndicesGetSettingsUrlParams::Name(name.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for IndicesGetSettingsRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesExistsTemplateUrlParams<'a> {
        Name(Name<'a>),
    }
    impl<'a> IndicesExistsTemplateUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesExistsTemplateUrlParams::Name(ref name) => {
                    let mut url = String::with_capacity(11usize + name.len());
                    url.push_str("/_template/");
                    url.push_str(name.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Head: /_template/{name}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html)"]
    pub struct IndicesExistsTemplateRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> IndicesExistsTemplateRequest<'a> {
        #[doc = "Request to: `/_template/{name}`"]
        pub fn for_name<IName>(name: IName) -> Self
        where
            IName: Into<Name<'a>>,
        {
            IndicesExistsTemplateRequest {
                url: IndicesExistsTemplateUrlParams::Name(name.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for IndicesExistsTemplateRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::HEAD,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SnapshotRestoreUrlParams<'a> {
        RepositorySnapshot(Repository<'a>, Snapshot<'a>),
    }
    impl<'a> SnapshotRestoreUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                SnapshotRestoreUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
                    let mut url =
                        String::with_capacity(21usize + repository.len() + snapshot.len());
                    url.push_str("/_snapshot/");
                    url.push_str(repository.as_ref());
                    url.push_str("/");
                    url.push_str(snapshot.as_ref());
                    url.push_str("/_restore");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_snapshot/{repository}/{snapshot}/_restore`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html)"]
    pub struct SnapshotRestoreRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> SnapshotRestoreRequest<'a, B> {
        #[doc = "Request to: `/_snapshot/{repository}/{snapshot}/_restore`"]
        pub fn for_repository_snapshot<IRepository, ISnapshot>(
            repository: IRepository,
            snapshot: ISnapshot,
            body: B,
        ) -> Self
        where
            IRepository: Into<Repository<'a>>,
            ISnapshot: Into<Snapshot<'a>>,
        {
            SnapshotRestoreRequest {
                url: SnapshotRestoreUrlParams::RepositorySnapshot(
                    repository.into(),
                    snapshot.into(),
                )
                .url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for SnapshotRestoreRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SnapshotGetUrlParams<'a> {
        RepositorySnapshot(Repository<'a>, Snapshot<'a>),
    }
    impl<'a> SnapshotGetUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                SnapshotGetUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
                    let mut url =
                        String::with_capacity(12usize + repository.len() + snapshot.len());
                    url.push_str("/_snapshot/");
                    url.push_str(repository.as_ref());
                    url.push_str("/");
                    url.push_str(snapshot.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_snapshot/{repository}/{snapshot}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html)"]
    pub struct SnapshotGetRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> SnapshotGetRequest<'a> {
        #[doc = "Request to: `/_snapshot/{repository}/{snapshot}`"]
        pub fn for_repository_snapshot<IRepository, ISnapshot>(
            repository: IRepository,
            snapshot: ISnapshot,
        ) -> Self
        where
            IRepository: Into<Repository<'a>>,
            ISnapshot: Into<Snapshot<'a>>,
        {
            SnapshotGetRequest {
                url: SnapshotGetUrlParams::RepositorySnapshot(repository.into(), snapshot.into())
                    .url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for SnapshotGetRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum GetSourceUrlParams<'a> {
        IndexId(Index<'a>, Id<'a>),
        IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
    }
    impl<'a> GetSourceUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                GetSourceUrlParams::IndexId(ref index, ref id) => {
                    let mut url = String::with_capacity(10usize + index.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_source/");
                    url.push_str(id.as_ref());
                    UrlPath::from(url)
                }
                GetSourceUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                    let mut url =
                        String::with_capacity(11usize + index.len() + ty.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    url.push_str("/_source");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /{index}/_source/{id}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html)"]
    pub struct GetSourceRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> GetSourceRequest<'a> {
        #[doc = "Request to: `/{index}/_source/{id}`"]
        pub fn for_index_id<IIndex, IId>(index: IIndex, id: IId) -> Self
        where
            IIndex: Into<Index<'a>>,
            IId: Into<Id<'a>>,
        {
            GetSourceRequest {
                url: GetSourceUrlParams::IndexId(index.into(), id.into()).url(),
            }
        }
        #[doc = "Request to: `/{index}/{type}/{id}/_source`"]
        pub fn for_index_ty_id<IIndex, IType, IId>(index: IIndex, ty: IType, id: IId) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IId: Into<Id<'a>>,
        {
            GetSourceRequest {
                url: GetSourceUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for GetSourceRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatAliasesUrlParams<'a> {
        None,
        Name(Name<'a>),
    }
    impl<'a> CatAliasesUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                CatAliasesUrlParams::None => UrlPath::from("/_cat/aliases"),
                CatAliasesUrlParams::Name(ref name) => {
                    let mut url = String::with_capacity(14usize + name.len());
                    url.push_str("/_cat/aliases/");
                    url.push_str(name.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cat/aliases`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-alias.html)"]
    pub struct CatAliasesRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> CatAliasesRequest<'a> {
        #[doc = "Request to: `/_cat/aliases`"]
        pub fn new() -> Self {
            CatAliasesRequest {
                url: CatAliasesUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/_cat/aliases/{name}`"]
        pub fn for_name<IName>(name: IName) -> Self
        where
            IName: Into<Name<'a>>,
        {
            CatAliasesRequest {
                url: CatAliasesUrlParams::Name(name.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for CatAliasesRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CreateUrlParams<'a> {
        IndexId(Index<'a>, Id<'a>),
        IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
    }
    impl<'a> CreateUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                CreateUrlParams::IndexId(ref index, ref id) => {
                    let mut url = String::with_capacity(10usize + index.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_create/");
                    url.push_str(id.as_ref());
                    UrlPath::from(url)
                }
                CreateUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                    let mut url =
                        String::with_capacity(11usize + index.len() + ty.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    url.push_str("/_create");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /{index}/_create/{id}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-index_.html)"]
    pub struct CreateRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> CreateRequest<'a, B> {
        #[doc = "Request to: `/{index}/_create/{id}`"]
        pub fn for_index_id<IIndex, IId>(index: IIndex, id: IId, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IId: Into<Id<'a>>,
        {
            CreateRequest {
                url: CreateUrlParams::IndexId(index.into(), id.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/{type}/{id}/_create`"]
        pub fn for_index_ty_id<IIndex, IType, IId>(
            index: IIndex,
            ty: IType,
            id: IId,
            body: B,
        ) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IId: Into<Id<'a>>,
        {
            CreateRequest {
                url: CreateUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for CreateRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatRecoveryUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> CatRecoveryUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                CatRecoveryUrlParams::None => UrlPath::from("/_cat/recovery"),
                CatRecoveryUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(15usize + index.len());
                    url.push_str("/_cat/recovery/");
                    url.push_str(index.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cat/recovery`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-recovery.html)"]
    pub struct CatRecoveryRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> CatRecoveryRequest<'a> {
        #[doc = "Request to: `/_cat/recovery`"]
        pub fn new() -> Self {
            CatRecoveryRequest {
                url: CatRecoveryUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/_cat/recovery/{index}`"]
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            CatRecoveryRequest {
                url: CatRecoveryUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for CatRecoveryRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IngestPutPipelineUrlParams<'a> {
        Id(Id<'a>),
    }
    impl<'a> IngestPutPipelineUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IngestPutPipelineUrlParams::Id(ref id) => {
                    let mut url = String::with_capacity(18usize + id.len());
                    url.push_str("/_ingest/pipeline/");
                    url.push_str(id.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Put: /_ingest/pipeline/{id}`\n\n[Elasticsearch Documentation](https://www.elastic.co/guide/en/elasticsearch/reference/master/put-pipeline-api.html)"]
    pub struct IngestPutPipelineRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> IngestPutPipelineRequest<'a, B> {
        #[doc = "Request to: `/_ingest/pipeline/{id}`"]
        pub fn for_id<IId>(id: IId, body: B) -> Self
        where
            IId: Into<Id<'a>>,
        {
            IngestPutPipelineRequest {
                url: IngestPutPipelineUrlParams::Id(id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for IngestPutPipelineRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::PUT,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum NodesInfoUrlParams<'a> {
        None,
        Metric(Metric<'a>),
        NodeId(NodeId<'a>),
        NodeIdMetric(NodeId<'a>, Metric<'a>),
    }
    impl<'a> NodesInfoUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                NodesInfoUrlParams::None => UrlPath::from("/_nodes"),
                NodesInfoUrlParams::Metric(ref metric) => {
                    let mut url = String::with_capacity(8usize + metric.len());
                    url.push_str("/_nodes/");
                    url.push_str(metric.as_ref());
                    UrlPath::from(url)
                }
                NodesInfoUrlParams::NodeId(ref node_id) => {
                    let mut url = String::with_capacity(8usize + node_id.len());
                    url.push_str("/_nodes/");
                    url.push_str(node_id.as_ref());
                    UrlPath::from(url)
                }
                NodesInfoUrlParams::NodeIdMetric(ref node_id, ref metric) => {
                    let mut url = String::with_capacity(9usize + node_id.len() + metric.len());
                    url.push_str("/_nodes/");
                    url.push_str(node_id.as_ref());
                    url.push_str("/");
                    url.push_str(metric.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_nodes`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-info.html)"]
    pub struct NodesInfoRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> NodesInfoRequest<'a> {
        #[doc = "Request to: `/_nodes`"]
        pub fn new() -> Self {
            NodesInfoRequest {
                url: NodesInfoUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/_nodes/{metric}`"]
        pub fn for_metric<IMetric>(metric: IMetric) -> Self
        where
            IMetric: Into<Metric<'a>>,
        {
            NodesInfoRequest {
                url: NodesInfoUrlParams::Metric(metric.into()).url(),
            }
        }
        #[doc = "Request to: `/_nodes/{node_id}`"]
        pub fn for_node_id<INodeId>(node_id: INodeId) -> Self
        where
            INodeId: Into<NodeId<'a>>,
        {
            NodesInfoRequest {
                url: NodesInfoUrlParams::NodeId(node_id.into()).url(),
            }
        }
        #[doc = "Request to: `/_nodes/{node_id}/{metric}`"]
        pub fn for_node_id_metric<INodeId, IMetric>(node_id: INodeId, metric: IMetric) -> Self
        where
            INodeId: Into<NodeId<'a>>,
            IMetric: Into<Metric<'a>>,
        {
            NodesInfoRequest {
                url: NodesInfoUrlParams::NodeIdMetric(node_id.into(), metric.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for NodesInfoRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatAllocationUrlParams<'a> {
        None,
        NodeId(NodeId<'a>),
    }
    impl<'a> CatAllocationUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                CatAllocationUrlParams::None => UrlPath::from("/_cat/allocation"),
                CatAllocationUrlParams::NodeId(ref node_id) => {
                    let mut url = String::with_capacity(17usize + node_id.len());
                    url.push_str("/_cat/allocation/");
                    url.push_str(node_id.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cat/allocation`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-allocation.html)"]
    pub struct CatAllocationRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> CatAllocationRequest<'a> {
        #[doc = "Request to: `/_cat/allocation`"]
        pub fn new() -> Self {
            CatAllocationRequest {
                url: CatAllocationUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/_cat/allocation/{node_id}`"]
        pub fn for_node_id<INodeId>(node_id: INodeId) -> Self
        where
            INodeId: Into<NodeId<'a>>,
        {
            CatAllocationRequest {
                url: CatAllocationUrlParams::NodeId(node_id.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for CatAllocationRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatShardsUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> CatShardsUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                CatShardsUrlParams::None => UrlPath::from("/_cat/shards"),
                CatShardsUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(13usize + index.len());
                    url.push_str("/_cat/shards/");
                    url.push_str(index.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cat/shards`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-shards.html)"]
    pub struct CatShardsRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> CatShardsRequest<'a> {
        #[doc = "Request to: `/_cat/shards`"]
        pub fn new() -> Self {
            CatShardsRequest {
                url: CatShardsUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/_cat/shards/{index}`"]
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            CatShardsRequest {
                url: CatShardsUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for CatShardsRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesExistsAliasUrlParams<'a> {
        IndexName(Index<'a>, Name<'a>),
        Name(Name<'a>),
    }
    impl<'a> IndicesExistsAliasUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesExistsAliasUrlParams::IndexName(ref index, ref name) => {
                    let mut url = String::with_capacity(9usize + index.len() + name.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_alias/");
                    url.push_str(name.as_ref());
                    UrlPath::from(url)
                }
                IndicesExistsAliasUrlParams::Name(ref name) => {
                    let mut url = String::with_capacity(8usize + name.len());
                    url.push_str("/_alias/");
                    url.push_str(name.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Head: /_alias/{name}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html)"]
    pub struct IndicesExistsAliasRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> IndicesExistsAliasRequest<'a> {
        #[doc = "Request to: `/{index}/_alias/{name}`"]
        pub fn for_index_name<IIndex, IName>(index: IIndex, name: IName) -> Self
        where
            IIndex: Into<Index<'a>>,
            IName: Into<Name<'a>>,
        {
            IndicesExistsAliasRequest {
                url: IndicesExistsAliasUrlParams::IndexName(index.into(), name.into()).url(),
            }
        }
        #[doc = "Request to: `/_alias/{name}`"]
        pub fn for_name<IName>(name: IName) -> Self
        where
            IName: Into<Name<'a>>,
        {
            IndicesExistsAliasRequest {
                url: IndicesExistsAliasUrlParams::Name(name.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for IndicesExistsAliasRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::HEAD,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CountUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> CountUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                CountUrlParams::None => UrlPath::from("/_count"),
                CountUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(8usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_count");
                    UrlPath::from(url)
                }
                CountUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(9usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_count");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_count`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/search-count.html)"]
    pub struct CountRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> CountRequest<'a, B> {
        #[doc = "Request to: `/_count`"]
        pub fn new(body: B) -> Self {
            CountRequest {
                url: CountUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_count`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            CountRequest {
                url: CountUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/{type}/_count`"]
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            CountRequest {
                url: CountUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for CountRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IngestProcessorGrokUrlParams {
        None,
    }
    impl IngestProcessorGrokUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                IngestProcessorGrokUrlParams::None => UrlPath::from("/_ingest/processor/grok"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_ingest/processor/grok`\n\n[Elasticsearch Documentation](https://www.elastic.co/guide/en/elasticsearch/reference/master/grok-processor.html#grok-processor-rest-get)"]
    pub struct IngestProcessorGrokRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> IngestProcessorGrokRequest<'a> {
        #[doc = "Request to: `/_ingest/processor/grok`"]
        pub fn new() -> Self {
            IngestProcessorGrokRequest {
                url: IngestProcessorGrokUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for IngestProcessorGrokRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesValidateQueryUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> IndicesValidateQueryUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesValidateQueryUrlParams::None => UrlPath::from("/_validate/query"),
                IndicesValidateQueryUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(17usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_validate/query");
                    UrlPath::from(url)
                }
                IndicesValidateQueryUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(18usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_validate/query");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_validate/query`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/search-validate.html)"]
    pub struct IndicesValidateQueryRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesValidateQueryRequest<'a, B> {
        #[doc = "Request to: `/_validate/query`"]
        pub fn new(body: B) -> Self {
            IndicesValidateQueryRequest {
                url: IndicesValidateQueryUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_validate/query`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesValidateQueryRequest {
                url: IndicesValidateQueryUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/{type}/_validate/query`"]
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            IndicesValidateQueryRequest {
                url: IndicesValidateQueryUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for IndicesValidateQueryRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ExistsSourceUrlParams<'a> {
        IndexId(Index<'a>, Id<'a>),
        IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
    }
    impl<'a> ExistsSourceUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                ExistsSourceUrlParams::IndexId(ref index, ref id) => {
                    let mut url = String::with_capacity(10usize + index.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_source/");
                    url.push_str(id.as_ref());
                    UrlPath::from(url)
                }
                ExistsSourceUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                    let mut url =
                        String::with_capacity(11usize + index.len() + ty.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    url.push_str("/_source");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Head: /{index}/_source/{id}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html)"]
    pub struct ExistsSourceRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> ExistsSourceRequest<'a> {
        #[doc = "Request to: `/{index}/_source/{id}`"]
        pub fn for_index_id<IIndex, IId>(index: IIndex, id: IId) -> Self
        where
            IIndex: Into<Index<'a>>,
            IId: Into<Id<'a>>,
        {
            ExistsSourceRequest {
                url: ExistsSourceUrlParams::IndexId(index.into(), id.into()).url(),
            }
        }
        #[doc = "Request to: `/{index}/{type}/{id}/_source`"]
        pub fn for_index_ty_id<IIndex, IType, IId>(index: IIndex, ty: IType, id: IId) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IId: Into<Id<'a>>,
        {
            ExistsSourceRequest {
                url: ExistsSourceUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for ExistsSourceRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::HEAD,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatTasksUrlParams {
        None,
    }
    impl CatTasksUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                CatTasksUrlParams::None => UrlPath::from("/_cat/tasks"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cat/tasks`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html)"]
    pub struct CatTasksRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> CatTasksRequest<'a> {
        #[doc = "Request to: `/_cat/tasks`"]
        pub fn new() -> Self {
            CatTasksRequest {
                url: CatTasksUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for CatTasksRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ExistsUrlParams<'a> {
        IndexId(Index<'a>, Id<'a>),
        IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
    }
    impl<'a> ExistsUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                ExistsUrlParams::IndexId(ref index, ref id) => {
                    let mut url = String::with_capacity(7usize + index.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_doc/");
                    url.push_str(id.as_ref());
                    UrlPath::from(url)
                }
                ExistsUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                    let mut url = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Head: /{index}/_doc/{id}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html)"]
    pub struct ExistsRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> ExistsRequest<'a> {
        #[doc = "Request to: `/{index}/_doc/{id}`"]
        pub fn for_index_id<IIndex, IId>(index: IIndex, id: IId) -> Self
        where
            IIndex: Into<Index<'a>>,
            IId: Into<Id<'a>>,
        {
            ExistsRequest {
                url: ExistsUrlParams::IndexId(index.into(), id.into()).url(),
            }
        }
        #[doc = "Request to: `/{index}/{type}/{id}`"]
        pub fn for_index_ty_id<IIndex, IType, IId>(index: IIndex, ty: IType, id: IId) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IId: Into<Id<'a>>,
        {
            ExistsRequest {
                url: ExistsUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for ExistsRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::HEAD,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IngestSimulateUrlParams<'a> {
        None,
        Id(Id<'a>),
    }
    impl<'a> IngestSimulateUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IngestSimulateUrlParams::None => UrlPath::from("/_ingest/pipeline/_simulate"),
                IngestSimulateUrlParams::Id(ref id) => {
                    let mut url = String::with_capacity(28usize + id.len());
                    url.push_str("/_ingest/pipeline/");
                    url.push_str(id.as_ref());
                    url.push_str("/_simulate");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_ingest/pipeline/_simulate`\n\n[Elasticsearch Documentation](https://www.elastic.co/guide/en/elasticsearch/reference/master/simulate-pipeline-api.html)"]
    pub struct IngestSimulateRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> IngestSimulateRequest<'a, B> {
        #[doc = "Request to: `/_ingest/pipeline/_simulate`"]
        pub fn new(body: B) -> Self {
            IngestSimulateRequest {
                url: IngestSimulateUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/_ingest/pipeline/{id}/_simulate`"]
        pub fn for_id<IId>(id: IId, body: B) -> Self
        where
            IId: Into<Id<'a>>,
        {
            IngestSimulateRequest {
                url: IngestSimulateUrlParams::Id(id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for IngestSimulateRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum MgetUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> MgetUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                MgetUrlParams::None => UrlPath::from("/_mget"),
                MgetUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(7usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_mget");
                    UrlPath::from(url)
                }
                MgetUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(8usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_mget");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_mget`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-multi-get.html)"]
    pub struct MgetRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> MgetRequest<'a, B> {
        #[doc = "Request to: `/_mget`"]
        pub fn new(body: B) -> Self {
            MgetRequest {
                url: MgetUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_mget`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            MgetRequest {
                url: MgetUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/{type}/_mget`"]
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            MgetRequest {
                url: MgetUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for MgetRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesExistsUrlParams<'a> {
        Index(Index<'a>),
    }
    impl<'a> IndicesExistsUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesExistsUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(1usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Head: /{index}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-exists.html)"]
    pub struct IndicesExistsRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> IndicesExistsRequest<'a> {
        #[doc = "Request to: `/{index}`"]
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesExistsRequest {
                url: IndicesExistsUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for IndicesExistsRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::HEAD,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum GetScriptUrlParams<'a> {
        Id(Id<'a>),
    }
    impl<'a> GetScriptUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                GetScriptUrlParams::Id(ref id) => {
                    let mut url = String::with_capacity(10usize + id.len());
                    url.push_str("/_scripts/");
                    url.push_str(id.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_scripts/{id}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-scripting.html)"]
    pub struct GetScriptRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> GetScriptRequest<'a> {
        #[doc = "Request to: `/_scripts/{id}`"]
        pub fn for_id<IId>(id: IId) -> Self
        where
            IId: Into<Id<'a>>,
        {
            GetScriptRequest {
                url: GetScriptUrlParams::Id(id.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for GetScriptRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ReindexRethrottleUrlParams<'a> {
        TaskId(TaskId<'a>),
    }
    impl<'a> ReindexRethrottleUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                ReindexRethrottleUrlParams::TaskId(ref task_id) => {
                    let mut url = String::with_capacity(22usize + task_id.len());
                    url.push_str("/_reindex/");
                    url.push_str(task_id.as_ref());
                    url.push_str("/_rethrottle");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_reindex/{task_id}/_rethrottle`\n\n[Elasticsearch Documentation](https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-reindex.html)"]
    pub struct ReindexRethrottleRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> ReindexRethrottleRequest<'a, B> {
        #[doc = "Request to: `/_reindex/{task_id}/_rethrottle`"]
        pub fn for_task_id<ITaskId>(task_id: ITaskId, body: B) -> Self
        where
            ITaskId: Into<TaskId<'a>>,
        {
            ReindexRethrottleRequest {
                url: ReindexRethrottleUrlParams::TaskId(task_id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for ReindexRethrottleRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum UpdateByQueryRethrottleUrlParams<'a> {
        TaskId(TaskId<'a>),
    }
    impl<'a> UpdateByQueryRethrottleUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                UpdateByQueryRethrottleUrlParams::TaskId(ref task_id) => {
                    let mut url = String::with_capacity(30usize + task_id.len());
                    url.push_str("/_update_by_query/");
                    url.push_str(task_id.as_ref());
                    url.push_str("/_rethrottle");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_update_by_query/{task_id}/_rethrottle`\n\n[Elasticsearch Documentation](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-update-by-query.html)"]
    pub struct UpdateByQueryRethrottleRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> UpdateByQueryRethrottleRequest<'a, B> {
        #[doc = "Request to: `/_update_by_query/{task_id}/_rethrottle`"]
        pub fn for_task_id<ITaskId>(task_id: ITaskId, body: B) -> Self
        where
            ITaskId: Into<TaskId<'a>>,
        {
            UpdateByQueryRethrottleRequest {
                url: UpdateByQueryRethrottleUrlParams::TaskId(task_id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for UpdateByQueryRethrottleRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesDeleteAliasUrlParams<'a> {
        IndexName(Index<'a>, Name<'a>),
    }
    impl<'a> IndicesDeleteAliasUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesDeleteAliasUrlParams::IndexName(ref index, ref name) => {
                    let mut url = String::with_capacity(11usize + index.len() + name.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_aliases/");
                    url.push_str(name.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Delete: /{index}/_alias/{name}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html)"]
    pub struct IndicesDeleteAliasRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> IndicesDeleteAliasRequest<'a> {
        #[doc = "Request to: `/{index}/_aliases/{name}`"]
        pub fn for_index_name<IIndex, IName>(index: IIndex, name: IName) -> Self
        where
            IIndex: Into<Index<'a>>,
            IName: Into<Name<'a>>,
        {
            IndicesDeleteAliasRequest {
                url: IndicesDeleteAliasUrlParams::IndexName(index.into(), name.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for IndicesDeleteAliasRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::DELETE,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesPutMappingUrlParams<'a> {
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
        Type(Type<'a>),
    }
    impl<'a> IndicesPutMappingUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesPutMappingUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(10usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_mapping");
                    UrlPath::from(url)
                }
                IndicesPutMappingUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(12usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_mappings/");
                    url.push_str(ty.as_ref());
                    UrlPath::from(url)
                }
                IndicesPutMappingUrlParams::Type(ref ty) => {
                    let mut url = String::with_capacity(11usize + ty.len());
                    url.push_str("/_mappings/");
                    url.push_str(ty.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /{index}/{type}/_mapping`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-put-mapping.html)"]
    pub struct IndicesPutMappingRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesPutMappingRequest<'a, B> {
        #[doc = "Request to: `{index}/_mapping`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesPutMappingRequest {
                url: IndicesPutMappingUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_mappings/{type}`"]
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            IndicesPutMappingRequest {
                url: IndicesPutMappingUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/_mappings/{type}`"]
        pub fn for_ty<IType>(ty: IType, body: B) -> Self
        where
            IType: Into<Type<'a>>,
        {
            IndicesPutMappingRequest {
                url: IndicesPutMappingUrlParams::Type(ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for IndicesPutMappingRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum NodesUsageUrlParams<'a> {
        None,
        Metric(Metric<'a>),
        NodeId(NodeId<'a>),
        NodeIdMetric(NodeId<'a>, Metric<'a>),
    }
    impl<'a> NodesUsageUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                NodesUsageUrlParams::None => UrlPath::from("/_nodes/usage"),
                NodesUsageUrlParams::Metric(ref metric) => {
                    let mut url = String::with_capacity(14usize + metric.len());
                    url.push_str("/_nodes/usage/");
                    url.push_str(metric.as_ref());
                    UrlPath::from(url)
                }
                NodesUsageUrlParams::NodeId(ref node_id) => {
                    let mut url = String::with_capacity(14usize + node_id.len());
                    url.push_str("/_nodes/");
                    url.push_str(node_id.as_ref());
                    url.push_str("/usage");
                    UrlPath::from(url)
                }
                NodesUsageUrlParams::NodeIdMetric(ref node_id, ref metric) => {
                    let mut url = String::with_capacity(15usize + node_id.len() + metric.len());
                    url.push_str("/_nodes/");
                    url.push_str(node_id.as_ref());
                    url.push_str("/usage/");
                    url.push_str(metric.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_nodes/usage`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-usage.html)"]
    pub struct NodesUsageRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> NodesUsageRequest<'a> {
        #[doc = "Request to: `/_nodes/usage`"]
        pub fn new() -> Self {
            NodesUsageRequest {
                url: NodesUsageUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/_nodes/usage/{metric}`"]
        pub fn for_metric<IMetric>(metric: IMetric) -> Self
        where
            IMetric: Into<Metric<'a>>,
        {
            NodesUsageRequest {
                url: NodesUsageUrlParams::Metric(metric.into()).url(),
            }
        }
        #[doc = "Request to: `/_nodes/{node_id}/usage`"]
        pub fn for_node_id<INodeId>(node_id: INodeId) -> Self
        where
            INodeId: Into<NodeId<'a>>,
        {
            NodesUsageRequest {
                url: NodesUsageUrlParams::NodeId(node_id.into()).url(),
            }
        }
        #[doc = "Request to: `/_nodes/{node_id}/usage/{metric}`"]
        pub fn for_node_id_metric<INodeId, IMetric>(node_id: INodeId, metric: IMetric) -> Self
        where
            INodeId: Into<NodeId<'a>>,
            IMetric: Into<Metric<'a>>,
        {
            NodesUsageRequest {
                url: NodesUsageUrlParams::NodeIdMetric(node_id.into(), metric.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for NodesUsageRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesClearCacheUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesClearCacheUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesClearCacheUrlParams::None => UrlPath::from("/_cache/clear"),
                IndicesClearCacheUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(14usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_cache/clear");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_cache/clear`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-clearcache.html)"]
    pub struct IndicesClearCacheRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesClearCacheRequest<'a, B> {
        #[doc = "Request to: `/_cache/clear`"]
        pub fn new(body: B) -> Self {
            IndicesClearCacheRequest {
                url: IndicesClearCacheUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_cache/clear`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesClearCacheRequest {
                url: IndicesClearCacheUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for IndicesClearCacheRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SnapshotDeleteUrlParams<'a> {
        RepositorySnapshot(Repository<'a>, Snapshot<'a>),
    }
    impl<'a> SnapshotDeleteUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                SnapshotDeleteUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
                    let mut url =
                        String::with_capacity(12usize + repository.len() + snapshot.len());
                    url.push_str("/_snapshot/");
                    url.push_str(repository.as_ref());
                    url.push_str("/");
                    url.push_str(snapshot.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Delete: /_snapshot/{repository}/{snapshot}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html)"]
    pub struct SnapshotDeleteRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> SnapshotDeleteRequest<'a> {
        #[doc = "Request to: `/_snapshot/{repository}/{snapshot}`"]
        pub fn for_repository_snapshot<IRepository, ISnapshot>(
            repository: IRepository,
            snapshot: ISnapshot,
        ) -> Self
        where
            IRepository: Into<Repository<'a>>,
            ISnapshot: Into<Snapshot<'a>>,
        {
            SnapshotDeleteRequest {
                url: SnapshotDeleteUrlParams::RepositorySnapshot(
                    repository.into(),
                    snapshot.into(),
                )
                .url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for SnapshotDeleteRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::DELETE,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesShrinkUrlParams<'a> {
        IndexTarget(Index<'a>, Target<'a>),
    }
    impl<'a> IndicesShrinkUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesShrinkUrlParams::IndexTarget(ref index, ref target) => {
                    let mut url = String::with_capacity(10usize + index.len() + target.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_shrink/");
                    url.push_str(target.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /{index}/_shrink/{target}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-shrink-index.html)"]
    pub struct IndicesShrinkRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesShrinkRequest<'a, B> {
        #[doc = "Request to: `/{index}/_shrink/{target}`"]
        pub fn for_index_target<IIndex, ITarget>(index: IIndex, target: ITarget, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            ITarget: Into<Target<'a>>,
        {
            IndicesShrinkRequest {
                url: IndicesShrinkUrlParams::IndexTarget(index.into(), target.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for IndicesShrinkRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ExplainUrlParams<'a> {
        IndexId(Index<'a>, Id<'a>),
        IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
    }
    impl<'a> ExplainUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                ExplainUrlParams::IndexId(ref index, ref id) => {
                    let mut url = String::with_capacity(11usize + index.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_explain/");
                    url.push_str(id.as_ref());
                    UrlPath::from(url)
                }
                ExplainUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                    let mut url =
                        String::with_capacity(12usize + index.len() + ty.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    url.push_str("/_explain");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /{index}/_explain/{id}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/search-explain.html)"]
    pub struct ExplainRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> ExplainRequest<'a, B> {
        #[doc = "Request to: `/{index}/_explain/{id}`"]
        pub fn for_index_id<IIndex, IId>(index: IIndex, id: IId, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IId: Into<Id<'a>>,
        {
            ExplainRequest {
                url: ExplainUrlParams::IndexId(index.into(), id.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/{type}/{id}/_explain`"]
        pub fn for_index_ty_id<IIndex, IType, IId>(
            index: IIndex,
            ty: IType,
            id: IId,
            body: B,
        ) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IId: Into<Id<'a>>,
        {
            ExplainRequest {
                url: ExplainUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for ExplainRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SimpleSearchUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> SimpleSearchUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                SimpleSearchUrlParams::None => UrlPath::from("/_search"),
                SimpleSearchUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(9usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_search");
                    UrlPath::from(url)
                }
                SimpleSearchUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(10usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_search");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_search`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/search-search.html)"]
    pub struct SimpleSearchRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> SimpleSearchRequest<'a> {
        #[doc = "Request to: `/_search`"]
        pub fn new() -> Self {
            SimpleSearchRequest {
                url: SimpleSearchUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/{index}/_search`"]
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            SimpleSearchRequest {
                url: SimpleSearchUrlParams::Index(index.into()).url(),
            }
        }
        #[doc = "Request to: `/{index}/{type}/_search`"]
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            SimpleSearchRequest {
                url: SimpleSearchUrlParams::IndexType(index.into(), ty.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for SimpleSearchRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SearchUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> SearchUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                SearchUrlParams::None => UrlPath::from("/_search"),
                SearchUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(9usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_search");
                    UrlPath::from(url)
                }
                SearchUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(10usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_search");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_search`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/search-search.html)"]
    pub struct SearchRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> SearchRequest<'a, B> {
        #[doc = "Request to: `/_search`"]
        pub fn new(body: B) -> Self {
            SearchRequest {
                url: SearchUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_search`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            SearchRequest {
                url: SearchUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/{type}/_search`"]
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            SearchRequest {
                url: SearchUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for SearchRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatHelpUrlParams {
        None,
    }
    impl CatHelpUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                CatHelpUrlParams::None => UrlPath::from("/_cat"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cat`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cat.html)"]
    pub struct CatHelpRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> CatHelpRequest<'a> {
        #[doc = "Request to: `/_cat`"]
        pub fn new() -> Self {
            CatHelpRequest {
                url: CatHelpUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for CatHelpRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatIndicesUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> CatIndicesUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                CatIndicesUrlParams::None => UrlPath::from("/_cat/indices"),
                CatIndicesUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(14usize + index.len());
                    url.push_str("/_cat/indices/");
                    url.push_str(index.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cat/indices`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-indices.html)"]
    pub struct CatIndicesRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> CatIndicesRequest<'a> {
        #[doc = "Request to: `/_cat/indices`"]
        pub fn new() -> Self {
            CatIndicesRequest {
                url: CatIndicesUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/_cat/indices/{index}`"]
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            CatIndicesRequest {
                url: CatIndicesUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for CatIndicesRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesAnalyzeUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesAnalyzeUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesAnalyzeUrlParams::None => UrlPath::from("/_analyze"),
                IndicesAnalyzeUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(10usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_analyze");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_analyze`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-analyze.html)"]
    pub struct IndicesAnalyzeRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesAnalyzeRequest<'a, B> {
        #[doc = "Request to: `/_analyze`"]
        pub fn new(body: B) -> Self {
            IndicesAnalyzeRequest {
                url: IndicesAnalyzeUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_analyze`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesAnalyzeRequest {
                url: IndicesAnalyzeUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for IndicesAnalyzeRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndexUrlParams<'a> {
        Index(Index<'a>),
        IndexId(Index<'a>, Id<'a>),
        IndexType(Index<'a>, Type<'a>),
        IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
    }
    impl<'a> IndexUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndexUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(6usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_doc");
                    UrlPath::from(url)
                }
                IndexUrlParams::IndexId(ref index, ref id) => {
                    let mut url = String::with_capacity(7usize + index.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_doc/");
                    url.push_str(id.as_ref());
                    UrlPath::from(url)
                }
                IndexUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(2usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    UrlPath::from(url)
                }
                IndexUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                    let mut url = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /{index}/_doc`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-index_.html)"]
    pub struct IndexRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> IndexRequest<'a, B> {
        #[doc = "Request to: `/{index}/_doc`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndexRequest {
                url: IndexUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_doc/{id}`"]
        pub fn for_index_id<IIndex, IId>(index: IIndex, id: IId, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IId: Into<Id<'a>>,
        {
            IndexRequest {
                url: IndexUrlParams::IndexId(index.into(), id.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/{type}`"]
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            IndexRequest {
                url: IndexUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/{type}/{id}`"]
        pub fn for_index_ty_id<IIndex, IType, IId>(
            index: IIndex,
            ty: IType,
            id: IId,
            body: B,
        ) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IId: Into<Id<'a>>,
        {
            IndexRequest {
                url: IndexUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for IndexRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IngestDeletePipelineUrlParams<'a> {
        Id(Id<'a>),
    }
    impl<'a> IngestDeletePipelineUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IngestDeletePipelineUrlParams::Id(ref id) => {
                    let mut url = String::with_capacity(18usize + id.len());
                    url.push_str("/_ingest/pipeline/");
                    url.push_str(id.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Delete: /_ingest/pipeline/{id}`\n\n[Elasticsearch Documentation](https://www.elastic.co/guide/en/elasticsearch/reference/master/delete-pipeline-api.html)"]
    pub struct IngestDeletePipelineRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> IngestDeletePipelineRequest<'a> {
        #[doc = "Request to: `/_ingest/pipeline/{id}`"]
        pub fn for_id<IId>(id: IId) -> Self
        where
            IId: Into<Id<'a>>,
        {
            IngestDeletePipelineRequest {
                url: IngestDeletePipelineUrlParams::Id(id.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for IngestDeletePipelineRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::DELETE,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesCloseUrlParams<'a> {
        Index(Index<'a>),
    }
    impl<'a> IndicesCloseUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesCloseUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(8usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_close");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /{index}/_close`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-open-close.html)"]
    pub struct IndicesCloseRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesCloseRequest<'a, B> {
        #[doc = "Request to: `/{index}/_close`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesCloseRequest {
                url: IndicesCloseUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for IndicesCloseRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatPendingTasksUrlParams {
        None,
    }
    impl CatPendingTasksUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                CatPendingTasksUrlParams::None => UrlPath::from("/_cat/pending_tasks"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cat/pending_tasks`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-pending-tasks.html)"]
    pub struct CatPendingTasksRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> CatPendingTasksRequest<'a> {
        #[doc = "Request to: `/_cat/pending_tasks`"]
        pub fn new() -> Self {
            CatPendingTasksRequest {
                url: CatPendingTasksUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for CatPendingTasksRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesStatsUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexMetric(Index<'a>, Metric<'a>),
        Metric(Metric<'a>),
    }
    impl<'a> IndicesStatsUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesStatsUrlParams::None => UrlPath::from("/_stats"),
                IndicesStatsUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(8usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_stats");
                    UrlPath::from(url)
                }
                IndicesStatsUrlParams::IndexMetric(ref index, ref metric) => {
                    let mut url = String::with_capacity(9usize + index.len() + metric.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_stats/");
                    url.push_str(metric.as_ref());
                    UrlPath::from(url)
                }
                IndicesStatsUrlParams::Metric(ref metric) => {
                    let mut url = String::with_capacity(8usize + metric.len());
                    url.push_str("/_stats/");
                    url.push_str(metric.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_stats`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-stats.html)"]
    pub struct IndicesStatsRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> IndicesStatsRequest<'a> {
        #[doc = "Request to: `/_stats`"]
        pub fn new() -> Self {
            IndicesStatsRequest {
                url: IndicesStatsUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/{index}/_stats`"]
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesStatsRequest {
                url: IndicesStatsUrlParams::Index(index.into()).url(),
            }
        }
        #[doc = "Request to: `/{index}/_stats/{metric}`"]
        pub fn for_index_metric<IIndex, IMetric>(index: IIndex, metric: IMetric) -> Self
        where
            IIndex: Into<Index<'a>>,
            IMetric: Into<Metric<'a>>,
        {
            IndicesStatsRequest {
                url: IndicesStatsUrlParams::IndexMetric(index.into(), metric.into()).url(),
            }
        }
        #[doc = "Request to: `/_stats/{metric}`"]
        pub fn for_metric<IMetric>(metric: IMetric) -> Self
        where
            IMetric: Into<Metric<'a>>,
        {
            IndicesStatsRequest {
                url: IndicesStatsUrlParams::Metric(metric.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for IndicesStatsRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesRecoveryUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesRecoveryUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesRecoveryUrlParams::None => UrlPath::from("/_recovery"),
                IndicesRecoveryUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(11usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_recovery");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_recovery`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-recovery.html)"]
    pub struct IndicesRecoveryRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> IndicesRecoveryRequest<'a> {
        #[doc = "Request to: `/_recovery`"]
        pub fn new() -> Self {
            IndicesRecoveryRequest {
                url: IndicesRecoveryUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/{index}/_recovery`"]
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesRecoveryRequest {
                url: IndicesRecoveryUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for IndicesRecoveryRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum InfoUrlParams {
        None,
    }
    impl InfoUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                InfoUrlParams::None => UrlPath::from("/"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/)"]
    pub struct InfoRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> InfoRequest<'a> {
        #[doc = "Request to: `/`"]
        pub fn new() -> Self {
            InfoRequest {
                url: InfoUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for InfoRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ClusterHealthUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> ClusterHealthUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                ClusterHealthUrlParams::None => UrlPath::from("/_cluster/health"),
                ClusterHealthUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(17usize + index.len());
                    url.push_str("/_cluster/health/");
                    url.push_str(index.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cluster/health`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-health.html)"]
    pub struct ClusterHealthRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> ClusterHealthRequest<'a> {
        #[doc = "Request to: `/_cluster/health`"]
        pub fn new() -> Self {
            ClusterHealthRequest {
                url: ClusterHealthUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/_cluster/health/{index}`"]
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            ClusterHealthRequest {
                url: ClusterHealthUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for ClusterHealthRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum NodesReloadSecureSettingsUrlParams<'a> {
        None,
        NodeId(NodeId<'a>),
    }
    impl<'a> NodesReloadSecureSettingsUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                NodesReloadSecureSettingsUrlParams::None => {
                    UrlPath::from("/_nodes/reload_secure_settings")
                }
                NodesReloadSecureSettingsUrlParams::NodeId(ref node_id) => {
                    let mut url = String::with_capacity(31usize + node_id.len());
                    url.push_str("/_nodes/");
                    url.push_str(node_id.as_ref());
                    url.push_str("/reload_secure_settings");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_nodes/reload_secure_settings`\n\n[Elasticsearch Documentation](https://www.elastic.co/guide/en/elasticsearch/reference/master/secure-settings.html#reloadable-secure-settings)"]
    pub struct NodesReloadSecureSettingsRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> NodesReloadSecureSettingsRequest<'a, B> {
        #[doc = "Request to: `/_nodes/reload_secure_settings`"]
        pub fn new(body: B) -> Self {
            NodesReloadSecureSettingsRequest {
                url: NodesReloadSecureSettingsUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/_nodes/{node_id}/reload_secure_settings`"]
        pub fn for_node_id<INodeId>(node_id: INodeId, body: B) -> Self
        where
            INodeId: Into<NodeId<'a>>,
        {
            NodesReloadSecureSettingsRequest {
                url: NodesReloadSecureSettingsUrlParams::NodeId(node_id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for NodesReloadSecureSettingsRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum MtermvectorsUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> MtermvectorsUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                MtermvectorsUrlParams::None => UrlPath::from("/_mtermvectors"),
                MtermvectorsUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(15usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_mtermvectors");
                    UrlPath::from(url)
                }
                MtermvectorsUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(16usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_mtermvectors");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_mtermvectors`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-multi-termvectors.html)"]
    pub struct MtermvectorsRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> MtermvectorsRequest<'a, B> {
        #[doc = "Request to: `/_mtermvectors`"]
        pub fn new(body: B) -> Self {
            MtermvectorsRequest {
                url: MtermvectorsUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_mtermvectors`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            MtermvectorsRequest {
                url: MtermvectorsUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/{type}/_mtermvectors`"]
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            MtermvectorsRequest {
                url: MtermvectorsUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for MtermvectorsRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesOpenUrlParams<'a> {
        Index(Index<'a>),
    }
    impl<'a> IndicesOpenUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesOpenUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(7usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_open");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /{index}/_open`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-open-close.html)"]
    pub struct IndicesOpenRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesOpenRequest<'a, B> {
        #[doc = "Request to: `/{index}/_open`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesOpenRequest {
                url: IndicesOpenUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for IndicesOpenRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatPluginsUrlParams {
        None,
    }
    impl CatPluginsUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                CatPluginsUrlParams::None => UrlPath::from("/_cat/plugins"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cat/plugins`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-plugins.html)"]
    pub struct CatPluginsRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> CatPluginsRequest<'a> {
        #[doc = "Request to: `/_cat/plugins`"]
        pub fn new() -> Self {
            CatPluginsRequest {
                url: CatPluginsUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for CatPluginsRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum UpdateByQueryUrlParams<'a> {
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> UpdateByQueryUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                UpdateByQueryUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(18usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_update_by_query");
                    UrlPath::from(url)
                }
                UpdateByQueryUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(19usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_update_by_query");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /{index}/_update_by_query`\n\n[Elasticsearch Documentation](https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-update-by-query.html)"]
    pub struct UpdateByQueryRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> UpdateByQueryRequest<'a, B> {
        #[doc = "Request to: `/{index}/_update_by_query`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            UpdateByQueryRequest {
                url: UpdateByQueryUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/{type}/_update_by_query`"]
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            UpdateByQueryRequest {
                url: UpdateByQueryUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for UpdateByQueryRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatCountUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> CatCountUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                CatCountUrlParams::None => UrlPath::from("/_cat/count"),
                CatCountUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(12usize + index.len());
                    url.push_str("/_cat/count/");
                    url.push_str(index.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cat/count`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-count.html)"]
    pub struct CatCountRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> CatCountRequest<'a> {
        #[doc = "Request to: `/_cat/count`"]
        pub fn new() -> Self {
            CatCountRequest {
                url: CatCountUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/_cat/count/{index}`"]
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            CatCountRequest {
                url: CatCountUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for CatCountRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SnapshotVerifyRepositoryUrlParams<'a> {
        Repository(Repository<'a>),
    }
    impl<'a> SnapshotVerifyRepositoryUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                SnapshotVerifyRepositoryUrlParams::Repository(ref repository) => {
                    let mut url = String::with_capacity(19usize + repository.len());
                    url.push_str("/_snapshot/");
                    url.push_str(repository.as_ref());
                    url.push_str("/_verify");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_snapshot/{repository}/_verify`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html)"]
    pub struct SnapshotVerifyRepositoryRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> SnapshotVerifyRepositoryRequest<'a, B> {
        #[doc = "Request to: `/_snapshot/{repository}/_verify`"]
        pub fn for_repository<IRepository>(repository: IRepository, body: B) -> Self
        where
            IRepository: Into<Repository<'a>>,
        {
            SnapshotVerifyRepositoryRequest {
                url: SnapshotVerifyRepositoryUrlParams::Repository(repository.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for SnapshotVerifyRepositoryRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesSplitUrlParams<'a> {
        IndexTarget(Index<'a>, Target<'a>),
    }
    impl<'a> IndicesSplitUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesSplitUrlParams::IndexTarget(ref index, ref target) => {
                    let mut url = String::with_capacity(9usize + index.len() + target.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_split/");
                    url.push_str(target.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /{index}/_split/{target}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-split-index.html)"]
    pub struct IndicesSplitRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesSplitRequest<'a, B> {
        #[doc = "Request to: `/{index}/_split/{target}`"]
        pub fn for_index_target<IIndex, ITarget>(index: IIndex, target: ITarget, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            ITarget: Into<Target<'a>>,
        {
            IndicesSplitRequest {
                url: IndicesSplitUrlParams::IndexTarget(index.into(), target.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for IndicesSplitRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ClusterRerouteUrlParams {
        None,
    }
    impl ClusterRerouteUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                ClusterRerouteUrlParams::None => UrlPath::from("/_cluster/reroute"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_cluster/reroute`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-reroute.html)"]
    pub struct ClusterRerouteRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> ClusterRerouteRequest<'a, B> {
        #[doc = "Request to: `/_cluster/reroute`"]
        pub fn new(body: B) -> Self {
            ClusterRerouteRequest {
                url: ClusterRerouteUrlParams::None.url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for ClusterRerouteRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesPutAliasUrlParams<'a> {
        IndexName(Index<'a>, Name<'a>),
    }
    impl<'a> IndicesPutAliasUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesPutAliasUrlParams::IndexName(ref index, ref name) => {
                    let mut url = String::with_capacity(11usize + index.len() + name.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_aliases/");
                    url.push_str(name.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /{index}/_alias/{name}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html)"]
    pub struct IndicesPutAliasRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesPutAliasRequest<'a, B> {
        #[doc = "Request to: `/{index}/_aliases/{name}`"]
        pub fn for_index_name<IIndex, IName>(index: IIndex, name: IName, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IName: Into<Name<'a>>,
        {
            IndicesPutAliasRequest {
                url: IndicesPutAliasUrlParams::IndexName(index.into(), name.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for IndicesPutAliasRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ClusterPendingTasksUrlParams {
        None,
    }
    impl ClusterPendingTasksUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                ClusterPendingTasksUrlParams::None => UrlPath::from("/_cluster/pending_tasks"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cluster/pending_tasks`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-pending.html)"]
    pub struct ClusterPendingTasksRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> ClusterPendingTasksRequest<'a> {
        #[doc = "Request to: `/_cluster/pending_tasks`"]
        pub fn new() -> Self {
            ClusterPendingTasksRequest {
                url: ClusterPendingTasksUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for ClusterPendingTasksRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesDeleteTemplateUrlParams<'a> {
        Name(Name<'a>),
    }
    impl<'a> IndicesDeleteTemplateUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesDeleteTemplateUrlParams::Name(ref name) => {
                    let mut url = String::with_capacity(11usize + name.len());
                    url.push_str("/_template/");
                    url.push_str(name.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Delete: /_template/{name}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html)"]
    pub struct IndicesDeleteTemplateRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> IndicesDeleteTemplateRequest<'a> {
        #[doc = "Request to: `/_template/{name}`"]
        pub fn for_name<IName>(name: IName) -> Self
        where
            IName: Into<Name<'a>>,
        {
            IndicesDeleteTemplateRequest {
                url: IndicesDeleteTemplateUrlParams::Name(name.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for IndicesDeleteTemplateRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::DELETE,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SnapshotGetRepositoryUrlParams<'a> {
        None,
        Repository(Repository<'a>),
    }
    impl<'a> SnapshotGetRepositoryUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                SnapshotGetRepositoryUrlParams::None => UrlPath::from("/_snapshot"),
                SnapshotGetRepositoryUrlParams::Repository(ref repository) => {
                    let mut url = String::with_capacity(11usize + repository.len());
                    url.push_str("/_snapshot/");
                    url.push_str(repository.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_snapshot`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html)"]
    pub struct SnapshotGetRepositoryRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> SnapshotGetRepositoryRequest<'a> {
        #[doc = "Request to: `/_snapshot`"]
        pub fn new() -> Self {
            SnapshotGetRepositoryRequest {
                url: SnapshotGetRepositoryUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/_snapshot/{repository}`"]
        pub fn for_repository<IRepository>(repository: IRepository) -> Self
        where
            IRepository: Into<Repository<'a>>,
        {
            SnapshotGetRepositoryRequest {
                url: SnapshotGetRepositoryUrlParams::Repository(repository.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for SnapshotGetRepositoryRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum DeleteByQueryRethrottleUrlParams<'a> {
        TaskId(TaskId<'a>),
    }
    impl<'a> DeleteByQueryRethrottleUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                DeleteByQueryRethrottleUrlParams::TaskId(ref task_id) => {
                    let mut url = String::with_capacity(30usize + task_id.len());
                    url.push_str("/_delete_by_query/");
                    url.push_str(task_id.as_ref());
                    url.push_str("/_rethrottle");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_delete_by_query/{task_id}/_rethrottle`\n\n[Elasticsearch Documentation](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-delete-by-query.html)"]
    pub struct DeleteByQueryRethrottleRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> DeleteByQueryRethrottleRequest<'a, B> {
        #[doc = "Request to: `/_delete_by_query/{task_id}/_rethrottle`"]
        pub fn for_task_id<ITaskId>(task_id: ITaskId, body: B) -> Self
        where
            ITaskId: Into<TaskId<'a>>,
        {
            DeleteByQueryRethrottleRequest {
                url: DeleteByQueryRethrottleUrlParams::TaskId(task_id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for DeleteByQueryRethrottleRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatHealthUrlParams {
        None,
    }
    impl CatHealthUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                CatHealthUrlParams::None => UrlPath::from("/_cat/health"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cat/health`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-health.html)"]
    pub struct CatHealthRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> CatHealthRequest<'a> {
        #[doc = "Request to: `/_cat/health`"]
        pub fn new() -> Self {
            CatHealthRequest {
                url: CatHealthUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for CatHealthRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatRepositoriesUrlParams {
        None,
    }
    impl CatRepositoriesUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                CatRepositoriesUrlParams::None => UrlPath::from("/_cat/repositories"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cat/repositories`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-repositories.html)"]
    pub struct CatRepositoriesRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> CatRepositoriesRequest<'a> {
        #[doc = "Request to: `/_cat/repositories`"]
        pub fn new() -> Self {
            CatRepositoriesRequest {
                url: CatRepositoriesUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for CatRepositoriesRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatNodeattrsUrlParams {
        None,
    }
    impl CatNodeattrsUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                CatNodeattrsUrlParams::None => UrlPath::from("/_cat/nodeattrs"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cat/nodeattrs`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-nodeattrs.html)"]
    pub struct CatNodeattrsRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> CatNodeattrsRequest<'a> {
        #[doc = "Request to: `/_cat/nodeattrs`"]
        pub fn new() -> Self {
            CatNodeattrsRequest {
                url: CatNodeattrsUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for CatNodeattrsRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ClearScrollUrlParams<'a> {
        None,
        ScrollId(ScrollId<'a>),
    }
    impl<'a> ClearScrollUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                ClearScrollUrlParams::None => UrlPath::from("/_search/scroll"),
                ClearScrollUrlParams::ScrollId(ref scroll_id) => {
                    let mut url = String::with_capacity(16usize + scroll_id.len());
                    url.push_str("/_search/scroll/");
                    url.push_str(scroll_id.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Delete: /_search/scroll/{scroll_id}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/search-request-scroll.html)"]
    pub struct ClearScrollRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> ClearScrollRequest<'a, B> {
        #[doc = "Request to: `/_search/scroll`"]
        pub fn new(body: B) -> Self {
            ClearScrollRequest {
                url: ClearScrollUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/_search/scroll/{scroll_id}`"]
        pub fn for_scroll_id<IScrollId>(scroll_id: IScrollId, body: B) -> Self
        where
            IScrollId: Into<ScrollId<'a>>,
        {
            ClearScrollRequest {
                url: ClearScrollUrlParams::ScrollId(scroll_id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for ClearScrollRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::DELETE,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum TasksGetUrlParams<'a> {
        TaskId(TaskId<'a>),
    }
    impl<'a> TasksGetUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                TasksGetUrlParams::TaskId(ref task_id) => {
                    let mut url = String::with_capacity(8usize + task_id.len());
                    url.push_str("/_tasks/");
                    url.push_str(task_id.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_tasks/{task_id}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html)"]
    pub struct TasksGetRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> TasksGetRequest<'a> {
        #[doc = "Request to: `/_tasks/{task_id}`"]
        pub fn for_task_id<ITaskId>(task_id: ITaskId) -> Self
        where
            ITaskId: Into<TaskId<'a>>,
        {
            TasksGetRequest {
                url: TasksGetUrlParams::TaskId(task_id.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for TasksGetRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatThreadPoolUrlParams<'a> {
        None,
        ThreadPoolPatterns(ThreadPoolPatterns<'a>),
    }
    impl<'a> CatThreadPoolUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                CatThreadPoolUrlParams::None => UrlPath::from("/_cat/thread_pool"),
                CatThreadPoolUrlParams::ThreadPoolPatterns(ref thread_pool_patterns) => {
                    let mut url = String::with_capacity(18usize + thread_pool_patterns.len());
                    url.push_str("/_cat/thread_pool/");
                    url.push_str(thread_pool_patterns.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cat/thread_pool`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-thread-pool.html)"]
    pub struct CatThreadPoolRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> CatThreadPoolRequest<'a> {
        #[doc = "Request to: `/_cat/thread_pool`"]
        pub fn new() -> Self {
            CatThreadPoolRequest {
                url: CatThreadPoolUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/_cat/thread_pool/{thread_pool_patterns}`"]
        pub fn for_thread_pool_patterns<IThreadPoolPatterns>(
            thread_pool_patterns: IThreadPoolPatterns,
        ) -> Self
        where
            IThreadPoolPatterns: Into<ThreadPoolPatterns<'a>>,
        {
            CatThreadPoolRequest {
                url: CatThreadPoolUrlParams::ThreadPoolPatterns(thread_pool_patterns.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for CatThreadPoolRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SnapshotCreateRepositoryUrlParams<'a> {
        Repository(Repository<'a>),
    }
    impl<'a> SnapshotCreateRepositoryUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                SnapshotCreateRepositoryUrlParams::Repository(ref repository) => {
                    let mut url = String::with_capacity(11usize + repository.len());
                    url.push_str("/_snapshot/");
                    url.push_str(repository.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_snapshot/{repository}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html)"]
    pub struct SnapshotCreateRepositoryRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> SnapshotCreateRepositoryRequest<'a, B> {
        #[doc = "Request to: `/_snapshot/{repository}`"]
        pub fn for_repository<IRepository>(repository: IRepository, body: B) -> Self
        where
            IRepository: Into<Repository<'a>>,
        {
            SnapshotCreateRepositoryRequest {
                url: SnapshotCreateRepositoryUrlParams::Repository(repository.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for SnapshotCreateRepositoryRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum DeleteScriptUrlParams<'a> {
        Id(Id<'a>),
    }
    impl<'a> DeleteScriptUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                DeleteScriptUrlParams::Id(ref id) => {
                    let mut url = String::with_capacity(10usize + id.len());
                    url.push_str("/_scripts/");
                    url.push_str(id.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Delete: /_scripts/{id}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-scripting.html)"]
    pub struct DeleteScriptRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> DeleteScriptRequest<'a> {
        #[doc = "Request to: `/_scripts/{id}`"]
        pub fn for_id<IId>(id: IId) -> Self
        where
            IId: Into<Id<'a>>,
        {
            DeleteScriptRequest {
                url: DeleteScriptUrlParams::Id(id.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for DeleteScriptRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::DELETE,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesFlushSyncedUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesFlushSyncedUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesFlushSyncedUrlParams::None => UrlPath::from("/_flush/synced"),
                IndicesFlushSyncedUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(15usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_flush/synced");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_flush/synced`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-synced-flush.html)"]
    pub struct IndicesFlushSyncedRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesFlushSyncedRequest<'a, B> {
        #[doc = "Request to: `/_flush/synced`"]
        pub fn new(body: B) -> Self {
            IndicesFlushSyncedRequest {
                url: IndicesFlushSyncedUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_flush/synced`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesFlushSyncedRequest {
                url: IndicesFlushSyncedUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for IndicesFlushSyncedRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SnapshotDeleteRepositoryUrlParams<'a> {
        Repository(Repository<'a>),
    }
    impl<'a> SnapshotDeleteRepositoryUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                SnapshotDeleteRepositoryUrlParams::Repository(ref repository) => {
                    let mut url = String::with_capacity(11usize + repository.len());
                    url.push_str("/_snapshot/");
                    url.push_str(repository.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Delete: /_snapshot/{repository}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html)"]
    pub struct SnapshotDeleteRepositoryRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> SnapshotDeleteRepositoryRequest<'a> {
        #[doc = "Request to: `/_snapshot/{repository}`"]
        pub fn for_repository<IRepository>(repository: IRepository) -> Self
        where
            IRepository: Into<Repository<'a>>,
        {
            SnapshotDeleteRepositoryRequest {
                url: SnapshotDeleteRepositoryUrlParams::Repository(repository.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for SnapshotDeleteRepositoryRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::DELETE,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesShardStoresUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesShardStoresUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesShardStoresUrlParams::None => UrlPath::from("/_shard_stores"),
                IndicesShardStoresUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(15usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_shard_stores");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_shard_stores`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-shards-stores.html)"]
    pub struct IndicesShardStoresRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> IndicesShardStoresRequest<'a> {
        #[doc = "Request to: `/_shard_stores`"]
        pub fn new() -> Self {
            IndicesShardStoresRequest {
                url: IndicesShardStoresUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/{index}/_shard_stores`"]
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesShardStoresRequest {
                url: IndicesShardStoresUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for IndicesShardStoresRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ClusterStateUrlParams<'a> {
        None,
        Metric(Metric<'a>),
        MetricIndex(Metric<'a>, Index<'a>),
    }
    impl<'a> ClusterStateUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                ClusterStateUrlParams::None => UrlPath::from("/_cluster/state"),
                ClusterStateUrlParams::Metric(ref metric) => {
                    let mut url = String::with_capacity(16usize + metric.len());
                    url.push_str("/_cluster/state/");
                    url.push_str(metric.as_ref());
                    UrlPath::from(url)
                }
                ClusterStateUrlParams::MetricIndex(ref metric, ref index) => {
                    let mut url = String::with_capacity(17usize + metric.len() + index.len());
                    url.push_str("/_cluster/state/");
                    url.push_str(metric.as_ref());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cluster/state`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-state.html)"]
    pub struct ClusterStateRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> ClusterStateRequest<'a> {
        #[doc = "Request to: `/_cluster/state`"]
        pub fn new() -> Self {
            ClusterStateRequest {
                url: ClusterStateUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/_cluster/state/{metric}`"]
        pub fn for_metric<IMetric>(metric: IMetric) -> Self
        where
            IMetric: Into<Metric<'a>>,
        {
            ClusterStateRequest {
                url: ClusterStateUrlParams::Metric(metric.into()).url(),
            }
        }
        #[doc = "Request to: `/_cluster/state/{metric}/{index}`"]
        pub fn for_metric_index<IMetric, IIndex>(metric: IMetric, index: IIndex) -> Self
        where
            IMetric: Into<Metric<'a>>,
            IIndex: Into<Index<'a>>,
        {
            ClusterStateRequest {
                url: ClusterStateUrlParams::MetricIndex(metric.into(), index.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for ClusterStateRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum PutScriptUrlParams<'a> {
        Id(Id<'a>),
        IdContext(Id<'a>, Context<'a>),
    }
    impl<'a> PutScriptUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                PutScriptUrlParams::Id(ref id) => {
                    let mut url = String::with_capacity(10usize + id.len());
                    url.push_str("/_scripts/");
                    url.push_str(id.as_ref());
                    UrlPath::from(url)
                }
                PutScriptUrlParams::IdContext(ref id, ref context) => {
                    let mut url = String::with_capacity(11usize + id.len() + context.len());
                    url.push_str("/_scripts/");
                    url.push_str(id.as_ref());
                    url.push_str("/");
                    url.push_str(context.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_scripts/{id}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-scripting.html)"]
    pub struct PutScriptRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> PutScriptRequest<'a, B> {
        #[doc = "Request to: `/_scripts/{id}`"]
        pub fn for_id<IId>(id: IId, body: B) -> Self
        where
            IId: Into<Id<'a>>,
        {
            PutScriptRequest {
                url: PutScriptUrlParams::Id(id.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/_scripts/{id}/{context}`"]
        pub fn for_id_context<IId, IContext>(id: IId, context: IContext, body: B) -> Self
        where
            IId: Into<Id<'a>>,
            IContext: Into<Context<'a>>,
        {
            PutScriptRequest {
                url: PutScriptUrlParams::IdContext(id.into(), context.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for PutScriptRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ClusterPutSettingsUrlParams {
        None,
    }
    impl ClusterPutSettingsUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                ClusterPutSettingsUrlParams::None => UrlPath::from("/_cluster/settings"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Put: /_cluster/settings`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-update-settings.html)"]
    pub struct ClusterPutSettingsRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> ClusterPutSettingsRequest<'a, B> {
        #[doc = "Request to: `/_cluster/settings`"]
        pub fn new(body: B) -> Self {
            ClusterPutSettingsRequest {
                url: ClusterPutSettingsUrlParams::None.url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for ClusterPutSettingsRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::PUT,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum DeleteByQueryUrlParams<'a> {
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> DeleteByQueryUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                DeleteByQueryUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(18usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_delete_by_query");
                    UrlPath::from(url)
                }
                DeleteByQueryUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(19usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_delete_by_query");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /{index}/_delete_by_query`\n\n[Elasticsearch Documentation](https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-delete-by-query.html)"]
    pub struct DeleteByQueryRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> DeleteByQueryRequest<'a, B> {
        #[doc = "Request to: `/{index}/_delete_by_query`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            DeleteByQueryRequest {
                url: DeleteByQueryUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/{type}/_delete_by_query`"]
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            DeleteByQueryRequest {
                url: DeleteByQueryUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for DeleteByQueryRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ClusterRemoteInfoUrlParams {
        None,
    }
    impl ClusterRemoteInfoUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                ClusterRemoteInfoUrlParams::None => UrlPath::from("/_remote/info"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_remote/info`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-remote-info.html)"]
    pub struct ClusterRemoteInfoRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> ClusterRemoteInfoRequest<'a> {
        #[doc = "Request to: `/_remote/info`"]
        pub fn new() -> Self {
            ClusterRemoteInfoRequest {
                url: ClusterRemoteInfoUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for ClusterRemoteInfoRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatSegmentsUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> CatSegmentsUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                CatSegmentsUrlParams::None => UrlPath::from("/_cat/segments"),
                CatSegmentsUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(15usize + index.len());
                    url.push_str("/_cat/segments/");
                    url.push_str(index.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cat/segments`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-segments.html)"]
    pub struct CatSegmentsRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> CatSegmentsRequest<'a> {
        #[doc = "Request to: `/_cat/segments`"]
        pub fn new() -> Self {
            CatSegmentsRequest {
                url: CatSegmentsUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/_cat/segments/{index}`"]
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            CatSegmentsRequest {
                url: CatSegmentsUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for CatSegmentsRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesPutSettingsUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesPutSettingsUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesPutSettingsUrlParams::None => UrlPath::from("/_settings"),
                IndicesPutSettingsUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(11usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_settings");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Put: /_settings`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-update-settings.html)"]
    pub struct IndicesPutSettingsRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesPutSettingsRequest<'a, B> {
        #[doc = "Request to: `/_settings`"]
        pub fn new(body: B) -> Self {
            IndicesPutSettingsRequest {
                url: IndicesPutSettingsUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_settings`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesPutSettingsRequest {
                url: IndicesPutSettingsUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for IndicesPutSettingsRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::PUT,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum PingUrlParams {
        None,
    }
    impl PingUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                PingUrlParams::None => UrlPath::from("/"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/)"]
    pub struct PingRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> PingRequest<'a> {
        #[doc = "Request to: `/`"]
        pub fn new() -> Self {
            PingRequest {
                url: PingUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for PingRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum PingHeadUrlParams {
        None,
    }
    impl PingHeadUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                PingHeadUrlParams::None => UrlPath::from("/"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Head: /`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/)"]
    pub struct PingHeadRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> PingHeadRequest<'a> {
        #[doc = "Request to: `/`"]
        pub fn new() -> Self {
            PingHeadRequest {
                url: PingHeadUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for PingHeadRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::HEAD,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesCreateUrlParams<'a> {
        Index(Index<'a>),
    }
    impl<'a> IndicesCreateUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesCreateUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(1usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Put: /{index}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-create-index.html)"]
    pub struct IndicesCreateRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesCreateRequest<'a, B> {
        #[doc = "Request to: `/{index}`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesCreateRequest {
                url: IndicesCreateUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for IndicesCreateRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::PUT,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatTemplatesUrlParams<'a> {
        None,
        Name(Name<'a>),
    }
    impl<'a> CatTemplatesUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                CatTemplatesUrlParams::None => UrlPath::from("/_cat/templates"),
                CatTemplatesUrlParams::Name(ref name) => {
                    let mut url = String::with_capacity(16usize + name.len());
                    url.push_str("/_cat/templates/");
                    url.push_str(name.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cat/templates`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-templates.html)"]
    pub struct CatTemplatesRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> CatTemplatesRequest<'a> {
        #[doc = "Request to: `/_cat/templates`"]
        pub fn new() -> Self {
            CatTemplatesRequest {
                url: CatTemplatesUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/_cat/templates/{name}`"]
        pub fn for_name<IName>(name: IName) -> Self
        where
            IName: Into<Name<'a>>,
        {
            CatTemplatesRequest {
                url: CatTemplatesUrlParams::Name(name.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for CatTemplatesRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum NodesHotThreadsUrlParams<'a> {
        None,
        NodeId(NodeId<'a>),
    }
    impl<'a> NodesHotThreadsUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                NodesHotThreadsUrlParams::None => UrlPath::from("/_nodes/hot_threads"),
                NodesHotThreadsUrlParams::NodeId(ref node_id) => {
                    let mut url = String::with_capacity(20usize + node_id.len());
                    url.push_str("/_nodes/");
                    url.push_str(node_id.as_ref());
                    url.push_str("/hot_threads");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_nodes/hot_threads`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-hot-threads.html)"]
    pub struct NodesHotThreadsRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> NodesHotThreadsRequest<'a> {
        #[doc = "Request to: `/_nodes/hot_threads`"]
        pub fn new() -> Self {
            NodesHotThreadsRequest {
                url: NodesHotThreadsUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/_nodes/{node_id}/hot_threads`"]
        pub fn for_node_id<INodeId>(node_id: INodeId) -> Self
        where
            INodeId: Into<NodeId<'a>>,
        {
            NodesHotThreadsRequest {
                url: NodesHotThreadsUrlParams::NodeId(node_id.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for NodesHotThreadsRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum DeleteUrlParams<'a> {
        IndexId(Index<'a>, Id<'a>),
        IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
    }
    impl<'a> DeleteUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                DeleteUrlParams::IndexId(ref index, ref id) => {
                    let mut url = String::with_capacity(7usize + index.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_doc/");
                    url.push_str(id.as_ref());
                    UrlPath::from(url)
                }
                DeleteUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                    let mut url = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Delete: /{index}/_doc/{id}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-delete.html)"]
    pub struct DeleteRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> DeleteRequest<'a> {
        #[doc = "Request to: `/{index}/_doc/{id}`"]
        pub fn for_index_id<IIndex, IId>(index: IIndex, id: IId) -> Self
        where
            IIndex: Into<Index<'a>>,
            IId: Into<Id<'a>>,
        {
            DeleteRequest {
                url: DeleteUrlParams::IndexId(index.into(), id.into()).url(),
            }
        }
        #[doc = "Request to: `/{index}/{type}/{id}`"]
        pub fn for_index_ty_id<IIndex, IType, IId>(index: IIndex, ty: IType, id: IId) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IId: Into<Id<'a>>,
        {
            DeleteRequest {
                url: DeleteUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for DeleteRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::DELETE,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum GetUrlParams<'a> {
        IndexId(Index<'a>, Id<'a>),
        IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
    }
    impl<'a> GetUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                GetUrlParams::IndexId(ref index, ref id) => {
                    let mut url = String::with_capacity(7usize + index.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_doc/");
                    url.push_str(id.as_ref());
                    UrlPath::from(url)
                }
                GetUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                    let mut url = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /{index}/_doc/{id}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html)"]
    pub struct GetRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> GetRequest<'a> {
        #[doc = "Request to: `/{index}/_doc/{id}`"]
        pub fn for_index_id<IIndex, IId>(index: IIndex, id: IId) -> Self
        where
            IIndex: Into<Index<'a>>,
            IId: Into<Id<'a>>,
        {
            GetRequest {
                url: GetUrlParams::IndexId(index.into(), id.into()).url(),
            }
        }
        #[doc = "Request to: `/{index}/{type}/{id}`"]
        pub fn for_index_ty_id<IIndex, IType, IId>(index: IIndex, ty: IType, id: IId) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IId: Into<Id<'a>>,
        {
            GetRequest {
                url: GetUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for GetRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum TermvectorsUrlParams<'a> {
        Index(Index<'a>),
        IndexId(Index<'a>, Id<'a>),
        IndexType(Index<'a>, Type<'a>),
        IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
    }
    impl<'a> TermvectorsUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                TermvectorsUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(15usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_termvectors/");
                    UrlPath::from(url)
                }
                TermvectorsUrlParams::IndexId(ref index, ref id) => {
                    let mut url = String::with_capacity(15usize + index.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_termvectors/");
                    url.push_str(id.as_ref());
                    UrlPath::from(url)
                }
                TermvectorsUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(15usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_termvectors");
                    UrlPath::from(url)
                }
                TermvectorsUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                    let mut url =
                        String::with_capacity(16usize + index.len() + ty.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    url.push_str("/_termvectors");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /{index}/_termvectors/{id}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-termvectors.html)"]
    pub struct TermvectorsRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> TermvectorsRequest<'a, B> {
        #[doc = "Request to: `/{index}/_termvectors/`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            TermvectorsRequest {
                url: TermvectorsUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_termvectors/{id}`"]
        pub fn for_index_id<IIndex, IId>(index: IIndex, id: IId, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IId: Into<Id<'a>>,
        {
            TermvectorsRequest {
                url: TermvectorsUrlParams::IndexId(index.into(), id.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/{type}/_termvectors`"]
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            TermvectorsRequest {
                url: TermvectorsUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/{type}/{id}/_termvectors`"]
        pub fn for_index_ty_id<IIndex, IType, IId>(
            index: IIndex,
            ty: IType,
            id: IId,
            body: B,
        ) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IId: Into<Id<'a>>,
        {
            TermvectorsRequest {
                url: TermvectorsUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for TermvectorsRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IngestGetPipelineUrlParams<'a> {
        None,
        Id(Id<'a>),
    }
    impl<'a> IngestGetPipelineUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IngestGetPipelineUrlParams::None => UrlPath::from("/_ingest/pipeline"),
                IngestGetPipelineUrlParams::Id(ref id) => {
                    let mut url = String::with_capacity(18usize + id.len());
                    url.push_str("/_ingest/pipeline/");
                    url.push_str(id.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_ingest/pipeline/{id}`\n\n[Elasticsearch Documentation](https://www.elastic.co/guide/en/elasticsearch/reference/master/get-pipeline-api.html)"]
    pub struct IngestGetPipelineRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> IngestGetPipelineRequest<'a> {
        #[doc = "Request to: `/_ingest/pipeline`"]
        pub fn new() -> Self {
            IngestGetPipelineRequest {
                url: IngestGetPipelineUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/_ingest/pipeline/{id}`"]
        pub fn for_id<IId>(id: IId) -> Self
        where
            IId: Into<Id<'a>>,
        {
            IngestGetPipelineRequest {
                url: IngestGetPipelineUrlParams::Id(id.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for IngestGetPipelineRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ClusterAllocationExplainUrlParams {
        None,
    }
    impl ClusterAllocationExplainUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                ClusterAllocationExplainUrlParams::None => {
                    UrlPath::from("/_cluster/allocation/explain")
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_cluster/allocation/explain`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-allocation-explain.html)"]
    pub struct ClusterAllocationExplainRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> ClusterAllocationExplainRequest<'a, B> {
        #[doc = "Request to: `/_cluster/allocation/explain`"]
        pub fn new(body: B) -> Self {
            ClusterAllocationExplainRequest {
                url: ClusterAllocationExplainUrlParams::None.url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for ClusterAllocationExplainRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesDeleteUrlParams<'a> {
        Index(Index<'a>),
    }
    impl<'a> IndicesDeleteUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesDeleteUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(1usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Delete: /{index}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-delete-index.html)"]
    pub struct IndicesDeleteRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> IndicesDeleteRequest<'a> {
        #[doc = "Request to: `/{index}`"]
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesDeleteRequest {
                url: IndicesDeleteUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for IndicesDeleteRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::DELETE,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum BulkUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> BulkUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                BulkUrlParams::None => UrlPath::from("/_bulk"),
                BulkUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(7usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_bulk");
                    UrlPath::from(url)
                }
                BulkUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(8usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_bulk");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_bulk`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-bulk.html)"]
    pub struct BulkRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> BulkRequest<'a, B> {
        #[doc = "Request to: `/_bulk`"]
        pub fn new(body: B) -> Self {
            BulkRequest {
                url: BulkUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_bulk`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            BulkRequest {
                url: BulkUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/{type}/_bulk`"]
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            BulkRequest {
                url: BulkUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for BulkRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ClusterGetSettingsUrlParams {
        None,
    }
    impl ClusterGetSettingsUrlParams {
        pub fn url<'a>(self) -> UrlPath<'a> {
            match self {
                ClusterGetSettingsUrlParams::None => UrlPath::from("/_cluster/settings"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_cluster/settings`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-update-settings.html)"]
    pub struct ClusterGetSettingsRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> ClusterGetSettingsRequest<'a> {
        #[doc = "Request to: `/_cluster/settings`"]
        pub fn new() -> Self {
            ClusterGetSettingsRequest {
                url: ClusterGetSettingsUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for ClusterGetSettingsRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesPutTemplateUrlParams<'a> {
        Name(Name<'a>),
    }
    impl<'a> IndicesPutTemplateUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesPutTemplateUrlParams::Name(ref name) => {
                    let mut url = String::with_capacity(11usize + name.len());
                    url.push_str("/_template/");
                    url.push_str(name.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /_template/{name}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html)"]
    pub struct IndicesPutTemplateRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesPutTemplateRequest<'a, B> {
        #[doc = "Request to: `/_template/{name}`"]
        pub fn for_name<IName>(name: IName, body: B) -> Self
        where
            IName: Into<Name<'a>>,
        {
            IndicesPutTemplateRequest {
                url: IndicesPutTemplateUrlParams::Name(name.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for IndicesPutTemplateRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesGetUpgradeUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesGetUpgradeUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesGetUpgradeUrlParams::None => UrlPath::from("/_upgrade"),
                IndicesGetUpgradeUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(10usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_upgrade");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_upgrade`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-upgrade.html)"]
    pub struct IndicesGetUpgradeRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> IndicesGetUpgradeRequest<'a> {
        #[doc = "Request to: `/_upgrade`"]
        pub fn new() -> Self {
            IndicesGetUpgradeRequest {
                url: IndicesGetUpgradeUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/{index}/_upgrade`"]
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesGetUpgradeRequest {
                url: IndicesGetUpgradeUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for IndicesGetUpgradeRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesGetTemplateUrlParams<'a> {
        None,
        Name(Name<'a>),
    }
    impl<'a> IndicesGetTemplateUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                IndicesGetTemplateUrlParams::None => UrlPath::from("/_template"),
                IndicesGetTemplateUrlParams::Name(ref name) => {
                    let mut url = String::with_capacity(11usize + name.len());
                    url.push_str("/_template/");
                    url.push_str(name.as_ref());
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Get: /_template/{name}`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html)"]
    pub struct IndicesGetTemplateRequest<'a> {
        pub url: UrlPath<'a>,
    }
    impl<'a> IndicesGetTemplateRequest<'a> {
        #[doc = "Request to: `/_template`"]
        pub fn new() -> Self {
            IndicesGetTemplateRequest {
                url: IndicesGetTemplateUrlParams::None.url(),
            }
        }
        #[doc = "Request to: `/_template/{name}`"]
        pub fn for_name<IName>(name: IName) -> Self
        where
            IName: Into<Name<'a>>,
        {
            IndicesGetTemplateRequest {
                url: IndicesGetTemplateUrlParams::Name(name.into()).url(),
            }
        }
    }
    impl<'a> Into<Endpoint<'a, DefaultBody>> for IndicesGetTemplateRequest<'a> {
        fn into(self) -> Endpoint<'a, DefaultBody> {
            Endpoint {
                url: self.url,
                method: Method::GET,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SearchShardsUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> SearchShardsUrlParams<'a> {
        pub fn url(self) -> UrlPath<'a> {
            match self {
                SearchShardsUrlParams::None => UrlPath::from("/_search_shards"),
                SearchShardsUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(16usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_search_shards");
                    UrlPath::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = "`Post: /{index}/_search_shards`\n\n[Elasticsearch Documentation](http://www.elastic.co/guide/en/elasticsearch/reference/master/search-shards.html)"]
    pub struct SearchShardsRequest<'a, B> {
        pub url: UrlPath<'a>,
        pub body: B,
    }
    impl<'a, B> SearchShardsRequest<'a, B> {
        #[doc = "Request to: `/_search_shards`"]
        pub fn new(body: B) -> Self {
            SearchShardsRequest {
                url: SearchShardsUrlParams::None.url(),
                body: body,
            }
        }
        #[doc = "Request to: `/{index}/_search_shards`"]
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            SearchShardsRequest {
                url: SearchShardsUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<Endpoint<'a, B>> for SearchShardsRequest<'a, B> {
        fn into(self) -> Endpoint<'a, B> {
            Endpoint {
                url: self.url,
                method: Method::POST,
                body: Some(self.body),
            }
        }
    }
}

pub mod http {
    use std::{
        borrow::Cow,
        ops::Deref,
    };
    extern crate http;
    pub use self::http::Method;

    #[derive(Debug, PartialEq, Clone)]
    #[doc = r" A wrapper around an owned or borrowed url path."]
    pub struct UrlPath<'a>(Cow<'a, str>);
    impl<'a> From<&'a str> for UrlPath<'a> {
        fn from(value: &'a str) -> UrlPath<'a> {
            UrlPath(Cow::Borrowed(value))
        }
    }
    impl<'a> From<String> for UrlPath<'a> {
        fn from(value: String) -> UrlPath<'a> {
            UrlPath(Cow::Owned(value))
        }
    }
    impl<'a> Deref for UrlPath<'a> {
        type Target = Cow<'a, str>;
        fn deref(&self) -> &Cow<'a, str> {
            &self.0
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = r" A general request type that all endpoints can be converted into."]
    pub struct Endpoint<'a, B> {
        pub url: UrlPath<'a>,
        pub method: Method,
        pub body: Option<B>,
    }
    #[doc = r" A default body type."]
    pub type DefaultBody = &'static [u8];
    #[doc = r" A convenience method for a default, empty body."]
    #[doc = r" This method doesn't allocate."]
    pub fn empty_body() -> DefaultBody {
        &[]
    }
}

pub mod params {
    use std::borrow::Cow;

    include!("genned.params.rs");

    #[derive(Debug, PartialEq, Clone)]
    pub struct Alias<'a>(pub Cow<'a, str>);
    pub fn alias<'a, I>(value: I) -> Alias<'a>
    where
        I: Into<Alias<'a>>,
    {
        value.into()
    }
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
    impl<'a> From<Alias<'a>> for Cow<'a, str> {
        fn from(value: Alias<'a>) -> Cow<'a, str> {
            value.0
        }
    }
    impl<'a> ::std::ops::Deref for Alias<'a> {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Context<'a>(pub Cow<'a, str>);
    pub fn context<'a, I>(value: I) -> Context<'a>
    where
        I: Into<Context<'a>>,
    {
        value.into()
    }
    impl<'a> From<&'a str> for Context<'a> {
        fn from(value: &'a str) -> Context<'a> {
            Context(Cow::Borrowed(value))
        }
    }
    impl<'a> From<String> for Context<'a> {
        fn from(value: String) -> Context<'a> {
            Context(Cow::Owned(value))
        }
    }
    impl<'a> From<Context<'a>> for Cow<'a, str> {
        fn from(value: Context<'a>) -> Cow<'a, str> {
            value.0
        }
    }
    impl<'a> ::std::ops::Deref for Context<'a> {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Fields<'a>(pub Cow<'a, str>);
    pub fn fields<'a, I>(value: I) -> Fields<'a>
    where
        I: Into<Fields<'a>>,
    {
        value.into()
    }
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
    impl<'a> From<Fields<'a>> for Cow<'a, str> {
        fn from(value: Fields<'a>) -> Cow<'a, str> {
            value.0
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
    pub fn id<'a, I>(value: I) -> Id<'a>
    where
        I: Into<Id<'a>>,
    {
        value.into()
    }
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
    impl<'a> From<Id<'a>> for Cow<'a, str> {
        fn from(value: Id<'a>) -> Cow<'a, str> {
            value.0
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
    pub fn index<'a, I>(value: I) -> Index<'a>
    where
        I: Into<Index<'a>>,
    {
        value.into()
    }
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
    impl<'a> From<Index<'a>> for Cow<'a, str> {
        fn from(value: Index<'a>) -> Cow<'a, str> {
            value.0
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
    pub fn index_metric<'a, I>(value: I) -> IndexMetric<'a>
    where
        I: Into<IndexMetric<'a>>,
    {
        value.into()
    }
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
    impl<'a> From<IndexMetric<'a>> for Cow<'a, str> {
        fn from(value: IndexMetric<'a>) -> Cow<'a, str> {
            value.0
        }
    }
    impl<'a> ::std::ops::Deref for IndexMetric<'a> {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Metric<'a>(pub Cow<'a, str>);
    pub fn metric<'a, I>(value: I) -> Metric<'a>
    where
        I: Into<Metric<'a>>,
    {
        value.into()
    }
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
    impl<'a> From<Metric<'a>> for Cow<'a, str> {
        fn from(value: Metric<'a>) -> Cow<'a, str> {
            value.0
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
    pub fn name<'a, I>(value: I) -> Name<'a>
    where
        I: Into<Name<'a>>,
    {
        value.into()
    }
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
    impl<'a> From<Name<'a>> for Cow<'a, str> {
        fn from(value: Name<'a>) -> Cow<'a, str> {
            value.0
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
    pub fn new_index<'a, I>(value: I) -> NewIndex<'a>
    where
        I: Into<NewIndex<'a>>,
    {
        value.into()
    }
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
    impl<'a> From<NewIndex<'a>> for Cow<'a, str> {
        fn from(value: NewIndex<'a>) -> Cow<'a, str> {
            value.0
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
    pub fn node_id<'a, I>(value: I) -> NodeId<'a>
    where
        I: Into<NodeId<'a>>,
    {
        value.into()
    }
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
    impl<'a> From<NodeId<'a>> for Cow<'a, str> {
        fn from(value: NodeId<'a>) -> Cow<'a, str> {
            value.0
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
    pub fn repository<'a, I>(value: I) -> Repository<'a>
    where
        I: Into<Repository<'a>>,
    {
        value.into()
    }
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
    impl<'a> From<Repository<'a>> for Cow<'a, str> {
        fn from(value: Repository<'a>) -> Cow<'a, str> {
            value.0
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
    pub fn scroll_id<'a, I>(value: I) -> ScrollId<'a>
    where
        I: Into<ScrollId<'a>>,
    {
        value.into()
    }
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
    impl<'a> From<ScrollId<'a>> for Cow<'a, str> {
        fn from(value: ScrollId<'a>) -> Cow<'a, str> {
            value.0
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
    pub fn snapshot<'a, I>(value: I) -> Snapshot<'a>
    where
        I: Into<Snapshot<'a>>,
    {
        value.into()
    }
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
    impl<'a> From<Snapshot<'a>> for Cow<'a, str> {
        fn from(value: Snapshot<'a>) -> Cow<'a, str> {
            value.0
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
    pub fn target<'a, I>(value: I) -> Target<'a>
    where
        I: Into<Target<'a>>,
    {
        value.into()
    }
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
    impl<'a> From<Target<'a>> for Cow<'a, str> {
        fn from(value: Target<'a>) -> Cow<'a, str> {
            value.0
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
    pub fn task_id<'a, I>(value: I) -> TaskId<'a>
    where
        I: Into<TaskId<'a>>,
    {
        value.into()
    }
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
    impl<'a> From<TaskId<'a>> for Cow<'a, str> {
        fn from(value: TaskId<'a>) -> Cow<'a, str> {
            value.0
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
    pub fn thread_pool_patterns<'a, I>(value: I) -> ThreadPoolPatterns<'a>
    where
        I: Into<ThreadPoolPatterns<'a>>,
    {
        value.into()
    }
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
    impl<'a> From<ThreadPoolPatterns<'a>> for Cow<'a, str> {
        fn from(value: ThreadPoolPatterns<'a>) -> Cow<'a, str> {
            value.0
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
    pub fn ty<'a, I>(value: I) -> Type<'a>
    where
        I: Into<Type<'a>>,
    {
        value.into()
    }
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
    impl<'a> From<Type<'a>> for Cow<'a, str> {
        fn from(value: Type<'a>) -> Cow<'a, str> {
            value.0
        }
    }
    impl<'a> ::std::ops::Deref for Type<'a> {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

}
