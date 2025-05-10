const converter = new showdown.Converter();
const postGrid = document.getElementById('post-grid');
const contentDiv = document.getElementById('content');

const posts = [
    {
        title: '05-10-2025',
        path: 'posts/05-10-2025.md',
        icon: 'fa-code'
    }
];

const icons = [
    'fa-code',
    'fa-brain',
    'fa-server',
    'fa-mobile-alt',
    'fa-shield-alt',
    'fa-project-diagram'
];

function fetchPostList() {
    try {
        postGrid.innerHTML = '';

        if (posts.length === 0) {
            postGrid.innerHTML = '<div class="col-span-full text-center text-gray-400">Nenhum post disponível.</div>';
            return;
        }

        posts.forEach((post, index) => {
            const iconClass = post.icon || icons[index % icons.length];
            const postTitle = post.title;
            
            const card = document.createElement('div');
            card.className = 'bg-gray-900 p-8 rounded-xl shadow-lg hover:shadow-xl transition cursor-pointer';
            card.setAttribute('data-path', post.path);
            
            card.innerHTML = `
                <div class="w-14 h-14 bg-blue-900 rounded-lg flex items-center justify-center text-blue-300 mb-6">
                    <i class="fas ${iconClass} text-2xl"></i>
                </div>
                <h3 class="text-xl font-bold text-white mb-3">${postTitle}</h3>
                <p class="text-gray-400 mb-4">Clique para ler este artigo</p>
                <div class="flex items-center text-blue-400 text-sm">
                    <span>Publicado em ${new Date().toLocaleDateString('pt-BR')}</span>
                </div>
            `;
            
            card.addEventListener('click', () => {
                loadPostContent(post.path, postTitle);
            });
            
            postGrid.appendChild(card);
        });

    } catch (error) {
        console.error(error);
        postGrid.innerHTML = '<div class="col-span-full text-center text-gray-400">Erro ao carregar posts 😢</div>';
    }
}

async function loadPostContent(url, title) {
    try {
        const response = await fetch(url);
        if (!response.ok) throw new Error('Error loading post.');

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
        console.error(error);
        contentDiv.innerHTML = '<p class="text-red-400">Erro ao carregar conteúdo do post 😢</p>';
    }
}

// Carregar lista de posts quando a página carregar
document.addEventListener('DOMContentLoaded', fetchPostList);
