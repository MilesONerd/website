const converter = new showdown.Converter();
const postGrid = document.getElementById('post-grid');
const contentDiv = document.getElementById('content');

const icons = [
    'fa-code',
    'fa-brain',
    'fa-server',
    'fa-mobile-alt',
    'fa-shield-alt',
    'fa-project-diagram',
    'fa-laptop-code',
    'fa-database',
    'fa-cloud',
    'fa-network-wired'
];

async function fetchPostList() {
    try {
        postGrid.innerHTML = '<div class="col-span-full text-center text-gray-400">Carregando posts...</div>';
        
        const response = await fetch('/blog/posts.json');
        if (!response.ok) throw new Error('Error loading posts list');
        
        const posts = await response.json();
        
        if (posts.length === 0) {
            postGrid.innerHTML = '<div class="col-span-full text-center text-gray-400">Nenhum post disponível.</div>';
            return;
        }
        
        postGrid.innerHTML = '';
        
        posts.forEach((post, index) => {
            const iconClass = icons[index % icons.length];
            
            const card = document.createElement('div');
            card.className = 'bg-gray-900 p-8 rounded-xl shadow-lg hover:shadow-xl transition cursor-pointer';
            
            card.innerHTML = `
                <div class="w-14 h-14 bg-blue-900 rounded-lg flex items-center justify-center text-blue-300 mb-6">
                    <i class="fas ${iconClass} text-2xl"></i>
                </div>
                <h3 class="text-xl font-bold text-white mb-3">${post.title}</h3>
                <p class="text-gray-400 mb-4">Clique para ler este artigo</p>
                <div class="flex items-center text-blue-400 text-sm">
                    <span>Publicado em ${new Date(post.date).toLocaleDateString('pt-BR')}</span>
                </div>
            `;
            
            card.addEventListener('click', () => {
                loadPostContent(`/${post.path}`, post.title);
            });
            
            postGrid.appendChild(card);
        });
        
    } catch (error) {
        console.error('Error loading posts:', error);
        postGrid.innerHTML = '<div class="col-span-full text-center text-gray-400">Erro ao carregar posts 😢</div>';
    }
}

async function loadPostContent(path, title) {
    try {
        const response = await fetch(path);
        if (!response.ok) throw new Error('Error loading post content');
        
        const markdown = await response.text();
        const htmlContent = converter.makeHtml(markdown);
        
        contentDiv.classList.remove('hidden');
        contentDiv.innerHTML = `
            <h2 class="text-2xl font-bold text-white mb-4">${title}</h2>
            <div class="prose prose-invert max-w-none">${htmlContent}</div>
            <div class="mt-6">
                <button id="back-to-posts" class="px-4 py-2 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 transition">
                    Voltar para posts
                </button>
            </div>
        `;
        
        contentDiv.scrollIntoView({ behavior: 'smooth' });
        
        document.getElementById('back-to-posts').addEventListener('click', () => {
            contentDiv.classList.add('hidden');
            window.scrollTo({ top: 0, behavior: 'smooth' });
        });
        
    } catch (error) {
        console.error('Error loading post content:', error);
        contentDiv.innerHTML = '<p class="text-red-400">Erro ao carregar conteúdo do post 😢</p>';
    }
}

document.addEventListener('DOMContentLoaded', fetchPostList);
