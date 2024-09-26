struct TracingLayer {}

impl TracingLayer {
    fn new() -> Self {
        TracingLayer {}
    }
}

impl<S> tracing_subscriber::Layer<S> for TracingLayer where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>
{
    
}
